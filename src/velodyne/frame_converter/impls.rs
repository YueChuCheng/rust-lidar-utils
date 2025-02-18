use super::converter::RemainingPoints;
use crate::{
    common::*,
    velodyne::{
        marker::{ModelMarker, ReturnTypeMarker},
        packet::DataPacket,
        pcd_converter::PointCloudConverter,
        point::{
            DualReturnPoint, DynamicReturnFrame, DynamicReturnPoints, SingleReturnPoint,
            VelodynePoint,
        },
        LidarFrameMsg, PcdFrame,
    },
};

pub(crate) fn convert_single_return<PcdConverter, Model, ReturnType>(
    pcd_converter: &mut PcdConverter,
    remaining_points: &mut Vec<SingleReturnPoint>,
    packet: &DataPacket,
) -> Option<PcdFrame<SingleReturnPoint>>
where
    PcdConverter: PointCloudConverter<Model, ReturnType, Output = Vec<SingleReturnPoint>>,
    Model: ModelMarker,
    ReturnType: ReturnTypeMarker,
{
    let points = remaining_points
        .drain(..)
        .chain(pcd_converter.convert(packet).unwrap().into_iter());

    let (frames, new_remaining_points) = points_to_frames(points);
    let _ = mem::replace(remaining_points, new_remaining_points);
    frames
}

pub(crate) fn convert_dual_return<PcdConverter, Model, ReturnType>(
    pcd_converter: &mut PcdConverter,
    remaining_points: &mut Vec<DualReturnPoint>,
    packet: &DataPacket,
) -> Option<PcdFrame<DualReturnPoint>>
where
    PcdConverter: PointCloudConverter<Model, ReturnType, Output = Vec<DualReturnPoint>>,
    Model: ModelMarker,
    ReturnType: ReturnTypeMarker,
{
    let points = remaining_points
        .drain(..)
        .chain(pcd_converter.convert(packet).unwrap().into_iter());
    let (frames, new_remaining_points) = points_to_frames(points);
    let _ = mem::replace(remaining_points, new_remaining_points);
    frames
}

pub(crate) fn convert_dynamic_return<PcdConverter, Model, ReturnType>(
    pcd_converter: &mut PcdConverter,
    remaining_points: &mut RemainingPoints,
    packet: &DataPacket,
) -> Option<DynamicReturnFrame>
where
    PcdConverter: PointCloudConverter<Model, ReturnType, Output = DynamicReturnPoints>,
    Model: ModelMarker,
    ReturnType: ReturnTypeMarker,
{
    let new_points = pcd_converter.convert(packet).unwrap();
    match (remaining_points, new_points) {
        (
            RemainingPoints(DynamicReturnPoints::Single(remaining_points)),
            DynamicReturnPoints::Single(new_points),
        ) => {
            let points = remaining_points.drain(..).chain(new_points.into_iter());
            let (frame, new_remaining_points) = points_to_frames(points);
            let _ = mem::replace(remaining_points, new_remaining_points);
            if let Some(frame) = frame {
                return Some(DynamicReturnFrame::Single(frame));
            } else {
                return None;
            };
        }
        (
            RemainingPoints(DynamicReturnPoints::Dual(remaining_points)),
            DynamicReturnPoints::Dual(new_points),
        ) => {
            let points = remaining_points.drain(..).chain(new_points.into_iter());
            let (frame, new_remaining_points) = points_to_frames(points);
            let _ = mem::replace(remaining_points, new_remaining_points);

            if let Some(frame) = frame {
                return Some(DynamicReturnFrame::Dual(frame));
            } else {
                return None;
            };
        }
        _ => unreachable!(),
    }
}

fn points_to_frames<Point>(
    points: impl IntoIterator<Item = Point>,
) -> (Option<PcdFrame<Point>>, Vec<Point>)
where
    Point: VelodynePoint + LidarFrameMsg + Copy,
{
    let mut frames: Option<PcdFrame<Point>> = None;
    let mut remaining_points: Vec<Point> = vec![];
    let mut prev_azimuth = None;
    let mut remaining_channel: Vec<Point> = vec![];

    let mut prev_laser_id = u32::MIN;
    let mut col_idx_cnt = 0;

    let mut beam_num = 0;

    points.into_iter().for_each(|mut point| {
        let curr_azimuth = point.original_azimuth_angle();
        let pass_zero_azimuth = prev_azimuth.map_or(false, |prev| curr_azimuth < prev);

        // pass 0 azimuth, and remaining point need to be more than 0, in case the first few points is the left points of previous frame
        if pass_zero_azimuth && remaining_points.len() > 0 {
            let mut frame = PcdFrame::new();

            // sort channel order by row_idx
            for i in 0..(remaining_points.len() / beam_num) {
                remaining_points[i * beam_num..(i * beam_num + beam_num)]
                    .sort_by(|a, b| a.row_idx().partial_cmp(&b.row_idx()).unwrap());
            }

            frame.data.append(&mut remaining_points);
            frame.height = beam_num;
            frame.width = col_idx_cnt;
            frames = Some(frame);

            //reset line ID for new frame
            col_idx_cnt = 0;
        }

        if prev_laser_id > point.laser_id() {
            //previous data ID should either 31(for 32 beam laser) or 15(for 16 beam laser)
            assert!(prev_laser_id == 15 || prev_laser_id == 31);

            // input data length should be either 32 or 16
            assert!(remaining_channel.len() == 16 || remaining_channel.len() == 32);

            //count whether it is 32 beam or 16 beam
            beam_num = (prev_laser_id + 1) as usize;

            //append to remaining_points when a line is collected
            remaining_points.append(&mut remaining_channel);

            //update line ID for next line
            col_idx_cnt = col_idx_cnt + 1;
        }
        //set line ID
        point.set_col_idx(col_idx_cnt);
        prev_laser_id = point.laser_id();
        remaining_channel.push(point);
        prev_azimuth = Some(curr_azimuth);
    });

    let mut remain: Vec<Point> = vec![];
    remain.append(&mut remaining_points);
    remain.append(&mut remaining_channel);

    (frames, remain)
}

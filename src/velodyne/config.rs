//! Defines a set of Velodyne LiDAR configurations.

use super::{
    consts::{
        PUCK_HIRES_AZIMUTH_OFFSETS, PUCK_HIRES_ELEVAION_DEGREES, PUCK_HIRES_HORIZONTAL_OFFSETS,
        PUCK_HIRES_VERTICAL_OFFSETS, PUCK_LITE_AZIMUTH_OFFSETS, PUCK_LITE_ELEVAION_DEGREES,
        PUCK_LITE_HORIZONTAL_OFFSETS, PUCK_LITE_VERTICAL_OFFSETS, VLP_16_AZIMUTH_OFFSETS,
        VLP_16_ELEVAION_DEGREES, VLP_16_HORIZONTAL_OFFSETS, VLP_16_VERTICAL_OFFSETS,
        VLP_32C_AZIMUTH_OFFSETS, VLP_32C_ELEVAION_DEGREES, VLP_32C_HORIZONTAL_OFFSETS,
        VLP_32C_VERTICAL_OFFSETS,
    },
    marker::{
        DualReturn, DynamicReturn, LastReturn, ModelMarker, ReturnTypeMarker, StrongestReturn,
        Vlp16, Vlp32,
    },
    packet::ReturnMode,
};
use crate::common::*;

pub type Vlp16Config<ReturnType> = Config<Vlp16, ReturnType>;
pub type Vlp32Config<ReturnType> = Config<Vlp32, ReturnType>;

/// Config type for Velodyne LiDARs.
#[derive(Debug, Clone)]
pub struct Config<Model, ReturnType>
where
    Model: ModelMarker,
    ReturnType: ReturnTypeMarker,
{
    pub lasers: Model::ParamArray,
    pub return_type: ReturnType,
    pub distance_resolution: Length,
}

#[derive(Debug, Clone)]
pub struct LaserParameter {
    pub elevation_angle: Angle,
    pub azimuth_offset: Angle,
    pub vertical_offset: Length,
    pub horizontal_offset: Length,
}

/// Config builder that builds [Config](Config) type.
#[derive(Debug, Clone)]
pub struct ConfigBuilder {}

impl ConfigBuilder {
    fn vlp_16_laser_params() -> [LaserParameter; 16] {
        let mut params: [MaybeUninit<LaserParameter>; 16] =
            unsafe { MaybeUninit::uninit().assume_init() };
        izip!(
            params.iter_mut(),
            VLP_16_ELEVAION_DEGREES.iter(),
            VLP_16_VERTICAL_OFFSETS.iter(),
            VLP_16_HORIZONTAL_OFFSETS.iter(),
            VLP_16_AZIMUTH_OFFSETS.iter(),
        )
        .for_each(
            |(param, elevation_angle, vertical_offset, horizontal_offset, azimuth_offset)| {
                *param = MaybeUninit::new(LaserParameter {
                    elevation_angle: Angle::new::<degree>(*elevation_angle),
                    vertical_offset: Length::new::<millimeter>(*vertical_offset),
                    horizontal_offset: Length::new::<millimeter>(*horizontal_offset),
                    azimuth_offset: Angle::new::<degree>(*azimuth_offset),
                });
            },
        );

        unsafe { std::mem::transmute::<_, [LaserParameter; 16]>(params) }
    }

    fn puck_hires_laser_params() -> [LaserParameter; 16] {
        let mut params: [MaybeUninit<LaserParameter>; 16] =
            unsafe { MaybeUninit::uninit().assume_init() };
        izip!(
            params.iter_mut(),
            PUCK_HIRES_ELEVAION_DEGREES.iter(),
            PUCK_HIRES_VERTICAL_OFFSETS.iter(),
            PUCK_HIRES_HORIZONTAL_OFFSETS.iter(),
            PUCK_HIRES_AZIMUTH_OFFSETS.iter(),
        )
        .for_each(
            |(param, elevation_angle, vertical_offset, horizontal_offset, azimuth_offset)| {
                *param = MaybeUninit::new(LaserParameter {
                    elevation_angle: Angle::new::<degree>(*elevation_angle),
                    vertical_offset: Length::new::<millimeter>(*vertical_offset),
                    horizontal_offset: Length::new::<millimeter>(*horizontal_offset),
                    azimuth_offset: Angle::new::<degree>(*azimuth_offset),
                });
            },
        );

        unsafe { std::mem::transmute::<_, [LaserParameter; 16]>(params) }
    }

    fn puck_lite_laser_params() -> [LaserParameter; 16] {
        let mut params: [MaybeUninit<LaserParameter>; 16] =
            unsafe { MaybeUninit::uninit().assume_init() };
        izip!(
            params.iter_mut(),
            PUCK_LITE_ELEVAION_DEGREES.iter(),
            PUCK_LITE_VERTICAL_OFFSETS.iter(),
            PUCK_LITE_HORIZONTAL_OFFSETS.iter(),
            PUCK_LITE_AZIMUTH_OFFSETS.iter(),
        )
        .for_each(
            |(param, elevation_angle, vertical_offset, horizontal_offset, azimuth_offset)| {
                *param = MaybeUninit::new(LaserParameter {
                    elevation_angle: Angle::new::<degree>(*elevation_angle),
                    vertical_offset: Length::new::<millimeter>(*vertical_offset),
                    horizontal_offset: Length::new::<millimeter>(*horizontal_offset),
                    azimuth_offset: Angle::new::<degree>(*azimuth_offset),
                });
            },
        );

        unsafe { std::mem::transmute::<_, [LaserParameter; 16]>(params) }
    }

    fn vlp_32c_laser_params() -> [LaserParameter; 32] {
        let mut params: [MaybeUninit<LaserParameter>; 32] =
            unsafe { MaybeUninit::uninit().assume_init() };
        izip!(
            params.iter_mut(),
            VLP_32C_ELEVAION_DEGREES.iter(),
            VLP_32C_VERTICAL_OFFSETS.iter(),
            VLP_32C_HORIZONTAL_OFFSETS.iter(),
            VLP_32C_AZIMUTH_OFFSETS.iter(),
        )
        .for_each(
            |(param, elevation_angle, vertical_offset, horizontal_offset, azimuth_offset)| {
                *param = MaybeUninit::new(LaserParameter {
                    elevation_angle: Angle::new::<degree>(*elevation_angle),
                    vertical_offset: Length::new::<millimeter>(*vertical_offset),
                    horizontal_offset: Length::new::<millimeter>(*horizontal_offset),
                    azimuth_offset: Angle::new::<degree>(*azimuth_offset),
                });
            },
        );

        unsafe { std::mem::transmute::<_, [LaserParameter; 32]>(params) }
    }

    pub fn vlp_16_last_return() -> Vlp16Config<LastReturn> {
        Config {
            lasers: Self::vlp_16_laser_params(),
            distance_resolution: Length::new::<millimeter>(2.0),
            return_type: LastReturn,
        }
    }

    pub fn vlp_16_strongest_return() -> Vlp16Config<StrongestReturn> {
        Config {
            lasers: Self::vlp_16_laser_params(),
            distance_resolution: Length::new::<millimeter>(2.0),
            return_type: StrongestReturn,
        }
    }

    pub fn vlp_16_dual_return() -> Vlp16Config<DualReturn> {
        Config {
            lasers: Self::vlp_16_laser_params(),
            distance_resolution: Length::new::<millimeter>(2.0),
            return_type: DualReturn,
        }
    }

    pub fn vlp_16_dynamic_return(return_mode: ReturnMode) -> Vlp16Config<DynamicReturn> {
        Config {
            lasers: Self::vlp_16_laser_params(),
            distance_resolution: Length::new::<millimeter>(2.0),
            return_type: DynamicReturn::from(return_mode),
        }
    }

    pub fn puck_hires_last_return() -> Vlp16Config<LastReturn> {
        Config {
            lasers: Self::puck_hires_laser_params(),
            distance_resolution: Length::new::<millimeter>(2.0),
            return_type: LastReturn,
        }
    }

    pub fn puck_hires_strongest_return() -> Vlp16Config<StrongestReturn> {
        Config {
            lasers: Self::puck_hires_laser_params(),
            distance_resolution: Length::new::<millimeter>(2.0),
            return_type: StrongestReturn,
        }
    }

    pub fn puck_hires_dual_return() -> Vlp16Config<DualReturn> {
        Config {
            lasers: Self::puck_hires_laser_params(),
            distance_resolution: Length::new::<millimeter>(2.0),
            return_type: DualReturn,
        }
    }

    pub fn puck_hires_dynamic_return(return_mode: ReturnMode) -> Vlp16Config<DynamicReturn> {
        Config {
            lasers: Self::puck_hires_laser_params(),
            distance_resolution: Length::new::<millimeter>(2.0),
            return_type: DynamicReturn::from(return_mode),
        }
    }

    pub fn puck_lite_last_return() -> Vlp16Config<LastReturn> {
        Config {
            lasers: Self::puck_lite_laser_params(),
            distance_resolution: Length::new::<millimeter>(2.0),
            return_type: LastReturn,
        }
    }

    pub fn puck_lite_strongest_return() -> Vlp16Config<StrongestReturn> {
        Config {
            lasers: Self::puck_lite_laser_params(),
            distance_resolution: Length::new::<millimeter>(2.0),
            return_type: StrongestReturn,
        }
    }

    pub fn puck_lite_dual_return() -> Vlp16Config<DualReturn> {
        Config {
            lasers: Self::puck_lite_laser_params(),
            distance_resolution: Length::new::<millimeter>(2.0),
            return_type: DualReturn,
        }
    }

    pub fn puck_lite_dynamic_return(return_mode: ReturnMode) -> Vlp16Config<DynamicReturn> {
        Config {
            lasers: Self::puck_lite_laser_params(),
            distance_resolution: Length::new::<millimeter>(2.0),
            return_type: DynamicReturn::from(return_mode),
        }
    }

    pub fn vlp_32c_last_return() -> Vlp32Config<LastReturn> {
        Config {
            lasers: Self::vlp_32c_laser_params(),
            distance_resolution: Length::new::<millimeter>(4.0),
            return_type: LastReturn,
        }
    }

    pub fn vlp_32c_strongest_return() -> Vlp32Config<StrongestReturn> {
        Config {
            lasers: Self::vlp_32c_laser_params(),
            distance_resolution: Length::new::<millimeter>(4.0),
            return_type: StrongestReturn,
        }
    }

    pub fn vlp_32c_dual_return() -> Vlp32Config<DualReturn> {
        Config {
            lasers: Self::vlp_32c_laser_params(),
            distance_resolution: Length::new::<millimeter>(4.0),
            return_type: DualReturn,
        }
    }

    pub fn vlp_32c_dynamic_return(return_mode: ReturnMode) -> Vlp32Config<DynamicReturn> {
        Config {
            lasers: Self::vlp_32c_laser_params(),
            distance_resolution: Length::new::<millimeter>(4.0),
            return_type: DynamicReturn::from(return_mode),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamsConfig {
    lasers: Vec<LaserConfig>,
    num_lasers: usize,
    distance_resolution: f64,
}

impl ParamsConfig {
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut reader = BufReader::new(File::open(path)?);
        let config = Self::from_reader(&mut reader)?;
        Ok(config)
    }

    pub fn from_reader<R>(reader: &mut R) -> Result<Self>
    where
        R: Read,
    {
        let mut text = String::new();
        reader.read_to_string(&mut text)?;
        let config = Self::from_str(&text)?;
        Ok(config)
    }

    pub fn from_str(text: &str) -> Result<Self> {
        let config: Self = serde_yaml::from_str(text)?;
        ensure!(
            config.distance_resolution > 0.0,
            "distance_resolution must be positive"
        );
        ensure!(
            config.num_lasers == config.lasers.len(),
            "the number of element in lasers field does not match num_layers"
        );
        ensure!(
            {
                config
                    .lasers
                    .iter()
                    .enumerate()
                    .all(|(idx, params)| idx == params.laser_id)
            },
            "the laser_id in lasers field must be consecutively counted from 1"
        );
        Ok(config)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaserConfig {
    pub dist_correction: f64,
    pub dist_correction_x: f64,
    pub dist_correction_y: f64,
    pub focal_distance: f64,
    pub focal_slope: f64,
    pub horiz_offset_correction: Option<f64>,
    pub laser_id: usize,
    pub rot_correction: f64,
    pub vert_correction: f64,
    pub vert_offset_correction: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn buildin_config_test() -> Result<()> {
        let _: Config<Vlp16, LastReturn> = ConfigBuilder::vlp_16_last_return();
        let _: Config<Vlp16, StrongestReturn> = ConfigBuilder::vlp_16_strongest_return();
        let _: Config<Vlp16, DualReturn> = ConfigBuilder::vlp_16_dual_return();

        let _: Config<Vlp16, LastReturn> = ConfigBuilder::puck_hires_last_return();
        let _: Config<Vlp16, StrongestReturn> = ConfigBuilder::puck_hires_strongest_return();
        let _: Config<Vlp16, DualReturn> = ConfigBuilder::puck_hires_dual_return();

        let _: Config<Vlp16, LastReturn> = ConfigBuilder::puck_lite_last_return();
        let _: Config<Vlp16, StrongestReturn> = ConfigBuilder::puck_lite_strongest_return();
        let _: Config<Vlp16, DualReturn> = ConfigBuilder::puck_lite_dual_return();

        let _: Config<Vlp32, LastReturn> = ConfigBuilder::vlp_32c_last_return();
        let _: Config<Vlp32, StrongestReturn> = ConfigBuilder::vlp_32c_strongest_return();
        let _: Config<Vlp32, DualReturn> = ConfigBuilder::vlp_32c_dual_return();

        Ok(())
    }

    #[test]
    fn load_yaml_params_test() -> Result<()> {
        ParamsConfig::from_str(include_str!("params/32db.yaml"))?;
        ParamsConfig::from_str(include_str!("params/64e_s2.1-sztaki.yaml"))?;
        ParamsConfig::from_str(include_str!("params/64e_s3-xiesc.yaml"))?;
        ParamsConfig::from_str(include_str!("params/64e_utexas.yaml"))?;
        ParamsConfig::from_str(include_str!("params/VeloView-VLP-32C.yaml"))?;
        ParamsConfig::from_str(include_str!("params/VLP16db.yaml"))?;
        ParamsConfig::from_str(include_str!("params/VLP16_hires_db.yaml"))?;
        Ok(())
    }
}

//! Version-1 unit identifiers and conversion to [`UnitId`].
//!
//! Files use plain `snake_case` strings such as `"length_meter"` and `"none"`.
//! Each identifier maps explicitly to its current unit variant.
//!
//! The accepted identifiers are a frozen snapshot: additions to the runtime
//! unit catalog do not automatically become valid version-1 file identifiers.

use hrafnix_units::UnitId;
use serde::{Deserialize, Deserializer};

/// Unit identifiers accepted by version-1 files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(remote = "Self", rename_all = "snake_case")]
pub(crate) enum UnitIdEnumReaderV1 {
    /// `none`
    None,

    /// `length_meter`
    LengthMeter,
    /// `length_picometer`
    LengthPicometer,
    /// `length_nanometer`
    LengthNanometer,
    /// `length_micrometer`
    LengthMicrometer,
    /// `length_millimeter`
    LengthMillimeter,
    /// `length_centimeter`
    LengthCentimeter,
    /// `length_kilometer`
    LengthKilometer,
    /// `length_foot`
    LengthFoot,
    /// `length_inch`
    LengthInch,
    /// `length_yard`
    LengthYard,
    /// `length_mile`
    LengthMile,

    /// `mass_kilogram`
    MassKilogram,
    /// `mass_gram`
    MassGram,
    /// `mass_picogram`
    MassPicogram,
    /// `mass_nanogram`
    MassNanogram,
    /// `mass_microgram`
    MassMicrogram,
    /// `mass_milligram`
    MassMilligram,
    /// `mass_megagram`
    MassMegagram,
    /// `mass_tonne`
    MassTonne,
    /// `mass_pound`
    MassPound,
    /// `mass_ounce`
    MassOunce,
    /// `mass_stone`
    MassStone,

    /// `time_second`
    TimeSecond,
    /// `time_picosecond`
    TimePicosecond,
    /// `time_nanosecond`
    TimeNanosecond,
    /// `time_microsecond`
    TimeMicrosecond,
    /// `time_millisecond`
    TimeMillisecond,
    /// `time_kilosecond`
    TimeKilosecond,
    /// `time_minute`
    TimeMinute,
    /// `time_hour`
    TimeHour,
    /// `time_day`
    TimeDay,
    /// `time_week`
    TimeWeek,
    /// `time_year`
    TimeYear,

    /// `current_ampere`
    CurrentAmpere,
    /// `current_nanoampere`
    CurrentNanoampere,
    /// `current_microampere`
    CurrentMicroampere,
    /// `current_milliampere`
    CurrentMilliampere,
    /// `current_kiloampere`
    CurrentKiloampere,
    /// `current_megaampere`
    CurrentMegaampere,

    /// `temperature_kelvin`
    TemperatureKelvin,
    /// `temperature_nanokelvin`
    TemperatureNanokelvin,
    /// `temperature_microkelvin`
    TemperatureMicrokelvin,
    /// `temperature_millikelvin`
    TemperatureMillikelvin,
    /// `temperature_kilokelvin`
    TemperatureKilokelvin,
    /// `temperature_celsius`
    TemperatureCelsius,
    /// `temperature_fahrenheit`
    TemperatureFahrenheit,

    /// `amount_mole`
    AmountMole,
    /// `amount_picomole`
    AmountPicomole,
    /// `amount_nanomole`
    AmountNanomole,
    /// `amount_micromole`
    AmountMicromole,
    /// `amount_millimole`
    AmountMillimole,
    /// `amount_kilomole`
    AmountKilomole,

    /// `frequency_hertz`
    FrequencyHertz,
    /// `frequency_picohertz`
    FrequencyPicohertz,
    /// `frequency_nanohertz`
    FrequencyNanohertz,
    /// `frequency_microhertz`
    FrequencyMicrohertz,
    /// `frequency_millihertz`
    FrequencyMillihertz,
    /// `frequency_kilohertz`
    FrequencyKilohertz,
    /// `frequency_megahertz`
    FrequencyMegahertz,
    /// `frequency_gigahertz`
    FrequencyGigahertz,
    /// `frequency_terahertz`
    FrequencyTerahertz,

    /// `angle_radian`
    AngleRadian,
    /// `angle_picoradian`
    AnglePicoradian,
    /// `angle_nanoradian`
    AngleNanoradian,
    /// `angle_microradian`
    AngleMicroradian,
    /// `angle_milliradian`
    AngleMilliradian,
    /// `angle_kiloradian`
    AngleKiloradian,
    /// `angle_megaradian`
    AngleMegaradian,
    /// `angle_gigaradian`
    AngleGigaradian,
    /// `angle_teraradian`
    AngleTeraradian,
    /// `angle_degree`
    AngleDegree,
    /// `angle_turn`
    AngleTurn,
    /// `angle_arcminute`
    AngleArcminute,
    /// `angle_arcsecond`
    AngleArcsecond,

    /// `solid_angle_steradian`
    SolidAngleSteradian,
    /// `solid_angle_picosteradian`
    SolidAnglePicosteradian,
    /// `solid_angle_nanosteradian`
    SolidAngleNanosteradian,
    /// `solid_angle_microsteradian`
    SolidAngleMicrosteradian,
    /// `solid_angle_millisteradian`
    SolidAngleMillisteradian,

    /// `force_newton`
    ForceNewton,
    /// `force_piconewton`
    ForcePiconewton,
    /// `force_nanonewton`
    ForceNanonewton,
    /// `force_micronewton`
    ForceMicronewton,
    /// `force_millinewton`
    ForceMillinewton,
    /// `force_kilonewton`
    ForceKilonewton,
    /// `force_meganewton`
    ForceMeganewton,
    /// `force_giganewton`
    ForceGiganewton,
    /// `force_teranewton`
    ForceTeranewton,
    /// `force_kilogram_force`
    ForceKilogramForce,
    /// `force_pound_force`
    ForcePoundForce,

    /// `pressure_pascal`
    PressurePascal,
    /// `pressure_picopascal`
    PressurePicopascal,
    /// `pressure_nanopascal`
    PressureNanopascal,
    /// `pressure_micropascal`
    PressureMicropascal,
    /// `pressure_millipascal`
    PressureMillipascal,
    /// `pressure_kilopascal`
    PressureKilopascal,
    /// `pressure_megapascal`
    PressureMegapascal,
    /// `pressure_gigapascal`
    PressureGigapascal,
    /// `pressure_terapascal`
    PressureTerapascal,
    /// `pressure_bar`
    PressureBar,
    /// `pressure_millibar`
    PressureMillibar,
    /// `pressure_standard_atmosphere`
    PressureStandardAtmosphere,
    /// `pressure_pound_per_square_inch`
    PressurePoundPerSquareInch,
    /// `pressure_millimeter_of_mercury`
    PressureMillimeterOfMercury,

    /// `energy_joule`
    EnergyJoule,
    /// `energy_picojoule`
    EnergyPicojoule,
    /// `energy_nanojoule`
    EnergyNanojoule,
    /// `energy_microjoule`
    EnergyMicrojoule,
    /// `energy_millijoule`
    EnergyMillijoule,
    /// `energy_kilojoule`
    EnergyKilojoule,
    /// `energy_megajoule`
    EnergyMegajoule,
    /// `energy_gigajoule`
    EnergyGigajoule,
    /// `energy_terajoule`
    EnergyTerajoule,
    /// `energy_watt_hour`
    EnergyWattHour,
    /// `energy_kilowatt_hour`
    EnergyKilowattHour,
    /// `energy_electronvolt`
    EnergyElectronvolt,

    /// `power_watt`
    PowerWatt,
    /// `power_picowatt`
    PowerPicowatt,
    /// `power_nanowatt`
    PowerNanowatt,
    /// `power_microwatt`
    PowerMicrowatt,
    /// `power_milliwatt`
    PowerMilliwatt,
    /// `power_kilowatt`
    PowerKilowatt,
    /// `power_megawatt`
    PowerMegawatt,
    /// `power_gigawatt`
    PowerGigawatt,
    /// `power_terawatt`
    PowerTerawatt,
    /// `power_mechanical_horsepower`
    PowerMechanicalHorsepower,
    /// `power_metric_horsepower`
    PowerMetricHorsepower,

    /// `electric_charge_coulomb`
    ElectricChargeCoulomb,
    /// `electric_charge_picocoulomb`
    ElectricChargePicocoulomb,
    /// `electric_charge_nanocoulomb`
    ElectricChargeNanocoulomb,
    /// `electric_charge_microcoulomb`
    ElectricChargeMicrocoulomb,
    /// `electric_charge_millicoulomb`
    ElectricChargeMillicoulomb,
    /// `electric_charge_kilocoulomb`
    ElectricChargeKilocoulomb,
    /// `electric_charge_megacoulomb`
    ElectricChargeMegacoulomb,
    /// `electric_charge_gigacoulomb`
    ElectricChargeGigacoulomb,
    /// `electric_charge_teracoulomb`
    ElectricChargeTeracoulomb,
    /// `electric_charge_ampere_hour`
    ElectricChargeAmpereHour,
    /// `electric_charge_milliampere_hour`
    ElectricChargeMilliampereHour,

    /// `voltage_volt`
    VoltageVolt,
    /// `voltage_picovolt`
    VoltagePicovolt,
    /// `voltage_nanovolt`
    VoltageNanovolt,
    /// `voltage_microvolt`
    VoltageMicrovolt,
    /// `voltage_millivolt`
    VoltageMillivolt,
    /// `voltage_kilovolt`
    VoltageKilovolt,
    /// `voltage_megavolt`
    VoltageMegavolt,
    /// `voltage_gigavolt`
    VoltageGigavolt,
    /// `voltage_teravolt`
    VoltageTeravolt,

    /// `resistance_ohm`
    ResistanceOhm,
    /// `resistance_picoohm`
    ResistancePicoohm,
    /// `resistance_nanoohm`
    ResistanceNanoohm,
    /// `resistance_microohm`
    ResistanceMicroohm,
    /// `resistance_milliohm`
    ResistanceMilliohm,
    /// `resistance_kiloohm`
    ResistanceKiloohm,
    /// `resistance_megaohm`
    ResistanceMegaohm,
    /// `resistance_gigaohm`
    ResistanceGigaohm,
    /// `resistance_teraohm`
    ResistanceTeraohm,

    /// `conductance_siemens`
    ConductanceSiemens,
    /// `conductance_picosiemens`
    ConductancePicosiemens,
    /// `conductance_nanosiemens`
    ConductanceNanosiemens,
    /// `conductance_microsiemens`
    ConductanceMicrosiemens,
    /// `conductance_millisiemens`
    ConductanceMillisiemens,
    /// `conductance_kilosiemens`
    ConductanceKilosiemens,
    /// `conductance_megasiemens`
    ConductanceMegasiemens,
    /// `conductance_gigasiemens`
    ConductanceGigasiemens,
    /// `conductance_terasiemens`
    ConductanceTerasiemens,

    /// `capacitance_farad`
    CapacitanceFarad,
    /// `capacitance_picofarad`
    CapacitancePicofarad,
    /// `capacitance_nanofarad`
    CapacitanceNanofarad,
    /// `capacitance_microfarad`
    CapacitanceMicrofarad,
    /// `capacitance_millifarad`
    CapacitanceMillifarad,
    /// `capacitance_kilofarad`
    CapacitanceKilofarad,

    /// `inductance_henry`
    InductanceHenry,
    /// `inductance_picohenry`
    InductancePicohenry,
    /// `inductance_nanohenry`
    InductanceNanohenry,
    /// `inductance_microhenry`
    InductanceMicrohenry,
    /// `inductance_millihenry`
    InductanceMillihenry,

    /// `magnetic_flux_density_tesla`
    MagneticFluxDensityTesla,
    /// `magnetic_flux_density_picotesla`
    MagneticFluxDensityPicotesla,
    /// `magnetic_flux_density_nanotesla`
    MagneticFluxDensityNanotesla,
    /// `magnetic_flux_density_microtesla`
    MagneticFluxDensityMicrotesla,
    /// `magnetic_flux_density_millitesla`
    MagneticFluxDensityMillitesla,

    /// `magnetic_flux_weber`
    MagneticFluxWeber,
    /// `magnetic_flux_picoweber`
    MagneticFluxPicoweber,
    /// `magnetic_flux_nanoweber`
    MagneticFluxNanoweber,
    /// `magnetic_flux_microweber`
    MagneticFluxMicroweber,
    /// `magnetic_flux_milliweber`
    MagneticFluxMilliweber,
    /// `magnetic_flux_kiloweber`
    MagneticFluxKiloweber,
    /// `magnetic_flux_megaweber`
    MagneticFluxMegaweber,
    /// `magnetic_flux_gigaweber`
    MagneticFluxGigaweber,
    /// `magnetic_flux_teraweber`
    MagneticFluxTeraweber,

    /// `area_square_meter`
    AreaSquareMeter,
    /// `area_square_micrometer`
    AreaSquareMicrometer,
    /// `area_square_millimeter`
    AreaSquareMillimeter,
    /// `area_square_centimeter`
    AreaSquareCentimeter,
    /// `area_square_kilometer`
    AreaSquareKilometer,
    /// `area_square_foot`
    AreaSquareFoot,
    /// `area_square_inch`
    AreaSquareInch,
    /// `area_acre`
    AreaAcre,
    /// `area_hectare`
    AreaHectare,
    /// `area_square_mile`
    AreaSquareMile,

    /// `volume_cubic_meter`
    VolumeCubicMeter,
    /// `volume_cubic_centimeter`
    VolumeCubicCentimeter,
    /// `volume_cubic_millimeter`
    VolumeCubicMillimeter,
    /// `volume_liter`
    VolumeLiter,
    /// `volume_picoliter`
    VolumePicoliter,
    /// `volume_nanoliter`
    VolumeNanoliter,
    /// `volume_microliter`
    VolumeMicroliter,
    /// `volume_milliliter`
    VolumeMilliliter,
    /// `volume_kiloliter`
    VolumeKiloliter,
    /// `volume_megaliter`
    VolumeMegaliter,
    /// `volume_gallon`
    VolumeGallon,
    /// `volume_imperial_gallon`
    VolumeImperialGallon,
    /// `volume_fluid_ounce`
    VolumeFluidOunce,
    /// `volume_imperial_fluid_ounce`
    VolumeImperialFluidOunce,
    /// `volume_cup`
    VolumeCup,
    /// `volume_pint`
    VolumePint,
    /// `volume_quart`
    VolumeQuart,

    /// `speed_meter_per_second`
    SpeedMeterPerSecond,
    /// `speed_millimeter_per_second`
    SpeedMillimeterPerSecond,
    /// `speed_centimeter_per_second`
    SpeedCentimeterPerSecond,
    /// `speed_kilometer_per_hour`
    SpeedKilometerPerHour,
    /// `speed_inch_per_second`
    SpeedInchPerSecond,
    /// `speed_foot_per_second`
    SpeedFootPerSecond,
    /// `speed_mile_per_hour`
    SpeedMilePerHour,
    /// `speed_knot`
    SpeedKnot,

    /// `acceleration_meter_per_second_squared`
    AccelerationMeterPerSecondSquared,
    /// `acceleration_millimeter_per_second_squared`
    AccelerationMillimeterPerSecondSquared,
    /// `acceleration_centimeter_per_second_squared`
    AccelerationCentimeterPerSecondSquared,
    /// `acceleration_inch_per_second_squared`
    AccelerationInchPerSecondSquared,
    /// `acceleration_foot_per_second_squared`
    AccelerationFootPerSecondSquared,
    /// `acceleration_standard_gravity`
    AccelerationStandardGravity,

    /// `angular_velocity_radian_per_second`
    AngularVelocityRadianPerSecond,
    /// `angular_velocity_degree_per_second`
    AngularVelocityDegreePerSecond,
    /// `angular_velocity_revolution_per_minute`
    AngularVelocityRevolutionPerMinute,
    /// `angular_velocity_revolution_per_second`
    AngularVelocityRevolutionPerSecond,

    /// `torque_newton_meter`
    TorqueNewtonMeter,
    /// `torque_newton_millimeter`
    TorqueNewtonMillimeter,
    /// `torque_newton_centimeter`
    TorqueNewtonCentimeter,
    /// `torque_pound_force_inch`
    TorquePoundForceInch,
    /// `torque_pound_force_foot`
    TorquePoundForceFoot,

    /// `density_kilogram_per_cubic_meter`
    DensityKilogramPerCubicMeter,
    /// `density_gram_per_cubic_centimeter`
    DensityGramPerCubicCentimeter,
    /// `density_gram_per_liter`
    DensityGramPerLiter,
    /// `density_kilogram_per_liter`
    DensityKilogramPerLiter,
    /// `density_pound_per_cubic_foot`
    DensityPoundPerCubicFoot,

    /// `volume_flow_rate_cubic_meter_per_second`
    VolumeFlowRateCubicMeterPerSecond,
    /// `volume_flow_rate_liter_per_minute`
    VolumeFlowRateLiterPerMinute,

    /// `displacement_per_revolution_cubic_meter_per_revolution`
    DisplacementPerRevolutionCubicMeterPerRevolution,
    /// `displacement_per_revolution_cubic_centimeter_per_revolution`
    DisplacementPerRevolutionCubicCentimeterPerRevolution,

    /// `angular_frequency_radian_per_second`
    AngularFrequencyRadianPerSecond,
    /// `angular_frequency_picoradian_per_second`
    AngularFrequencyPicoradianPerSecond,
    /// `angular_frequency_nanoradian_per_second`
    AngularFrequencyNanoradianPerSecond,
    /// `angular_frequency_microradian_per_second`
    AngularFrequencyMicroradianPerSecond,
    /// `angular_frequency_milliradian_per_second`
    AngularFrequencyMilliradianPerSecond,
    /// `angular_frequency_kiloradian_per_second`
    AngularFrequencyKiloradianPerSecond,
    /// `angular_frequency_megaradian_per_second`
    AngularFrequencyMegaradianPerSecond,
    /// `angular_frequency_gigaradian_per_second`
    AngularFrequencyGigaradianPerSecond,
    /// `angular_frequency_teraradian_per_second`
    AngularFrequencyTeraradianPerSecond,

    /// `moment_of_inertia_kilogram_square_meter`
    MomentOfInertiaKilogramSquareMeter,
    /// `moment_of_inertia_kilogram_square_centimeter`
    MomentOfInertiaKilogramSquareCentimeter,
    /// `moment_of_inertia_gram_square_centimeter`
    MomentOfInertiaGramSquareCentimeter,

    /// `linear_stiffness_newton_per_meter`
    LinearStiffnessNewtonPerMeter,
    /// `linear_stiffness_newton_per_millimeter`
    LinearStiffnessNewtonPerMillimeter,
    /// `linear_stiffness_kilonewton_per_meter`
    LinearStiffnessKilonewtonPerMeter,

    /// `linear_damping_newton_second_per_meter`
    LinearDampingNewtonSecondPerMeter,
    /// `linear_damping_newton_second_per_millimeter`
    LinearDampingNewtonSecondPerMillimeter,
    /// `linear_damping_kilonewton_second_per_meter`
    LinearDampingKilonewtonSecondPerMeter,

    /// `rotational_stiffness_newton_meter_per_radian`
    RotationalStiffnessNewtonMeterPerRadian,
    /// `rotational_stiffness_newton_millimeter_per_radian`
    RotationalStiffnessNewtonMillimeterPerRadian,
    /// `rotational_stiffness_kilonewton_meter_per_radian`
    RotationalStiffnessKilonewtonMeterPerRadian,

    /// `rotational_damping_newton_meter_second_per_radian`
    RotationalDampingNewtonMeterSecondPerRadian,
    /// `rotational_damping_newton_millimeter_second_per_radian`
    RotationalDampingNewtonMillimeterSecondPerRadian,
    /// `rotational_damping_kilonewton_meter_second_per_radian`
    RotationalDampingKilonewtonMeterSecondPerRadian,

    /// `dynamic_viscosity_pascal_second`
    DynamicViscosityPascalSecond,
    /// `dynamic_viscosity_millipascal_second`
    DynamicViscosityMillipascalSecond,
    /// `dynamic_viscosity_poise`
    DynamicViscosityPoise,
    /// `dynamic_viscosity_centipoise`
    DynamicViscosityCentipoise,

    /// `kinematic_viscosity_square_meter_per_second`
    KinematicViscositySquareMeterPerSecond,
    /// `kinematic_viscosity_square_millimeter_per_second`
    KinematicViscositySquareMillimeterPerSecond,
    /// `kinematic_viscosity_stokes`
    KinematicViscosityStokes,
    /// `kinematic_viscosity_centistokes`
    KinematicViscosityCentistokes,

    /// `mass_flow_rate_kilogram_per_second`
    MassFlowRateKilogramPerSecond,
    /// `mass_flow_rate_gram_per_second`
    MassFlowRateGramPerSecond,
    /// `mass_flow_rate_kilogram_per_minute`
    MassFlowRateKilogramPerMinute,
    /// `mass_flow_rate_kilogram_per_hour`
    MassFlowRateKilogramPerHour,

    /// `momentum_kilogram_meter_per_second`
    MomentumKilogramMeterPerSecond,
    /// `momentum_gram_centimeter_per_second`
    MomentumGramCentimeterPerSecond,
    /// `momentum_newton_second`
    MomentumNewtonSecond,

    /// `hydraulic_leakage_coefficient_cubic_meter_per_second_per_pascal`
    HydraulicLeakageCoefficientCubicMeterPerSecondPerPascal,
    /// `hydraulic_leakage_coefficient_liter_per_minute_per_bar`
    HydraulicLeakageCoefficientLiterPerMinutePerBar,

    /// `specific_heat_capacity_joule_per_kilogram_kelvin`
    SpecificHeatCapacityJoulePerKilogramKelvin,
    /// `specific_heat_capacity_kilojoule_per_kilogram_kelvin`
    SpecificHeatCapacityKilojoulePerKilogramKelvin,

    /// `specific_gas_constant_joule_per_kilogram_kelvin`
    SpecificGasConstantJoulePerKilogramKelvin,
    /// `specific_gas_constant_kilojoule_per_kilogram_kelvin`
    SpecificGasConstantKilojoulePerKilogramKelvin,

    /// `thermal_conductance_watt_per_kelvin`
    ThermalConductanceWattPerKelvin,
    /// `thermal_conductance_milliwatt_per_kelvin`
    ThermalConductanceMilliwattPerKelvin,
    /// `thermal_conductance_kilowatt_per_kelvin`
    ThermalConductanceKilowattPerKelvin,

    /// `hydraulic_resistance_pascal_second_per_cubic_meter`
    HydraulicResistancePascalSecondPerCubicMeter,
    /// `hydraulic_resistance_bar_minute_per_liter`
    HydraulicResistanceBarMinutePerLiter,

    /// `pneumatic_characteristic_impedance_pascal_second_per_joule`
    PneumaticCharacteristicImpedancePascalSecondPerJoule,
    /// `pneumatic_characteristic_impedance_second_per_cubic_meter`
    PneumaticCharacteristicImpedanceSecondPerCubicMeter,

    /// `turbulent_flow_coefficient_cubic_meter_per_second_per_square_root_pascal`
    TurbulentFlowCoefficientCubicMeterPerSecondPerSquareRootPascal,
    /// `turbulent_flow_coefficient_liter_per_minute_per_square_root_bar`
    TurbulentFlowCoefficientLiterPerMinutePerSquareRootBar,

    /// `motor_back_emf_constant_volt_second_per_radian`
    MotorBackEmfConstantVoltSecondPerRadian,
    /// `motor_back_emf_constant_millivolt_second_per_radian`
    MotorBackEmfConstantMillivoltSecondPerRadian,
    /// `motor_back_emf_constant_volt_per_revolution_per_minute`
    MotorBackEmfConstantVoltPerRevolutionPerMinute,

    /// `screw_pitch_meter_per_radian`
    ScrewPitchMeterPerRadian,
    /// `screw_pitch_millimeter_per_radian`
    ScrewPitchMillimeterPerRadian,
    /// `screw_pitch_millimeter_per_revolution`
    ScrewPitchMillimeterPerRevolution,

    /// `thrust_specific_fuel_consumption_kilogram_per_newton_second`
    ThrustSpecificFuelConsumptionKilogramPerNewtonSecond,
    /// `thrust_specific_fuel_consumption_kilogram_per_kilonewton_hour`
    ThrustSpecificFuelConsumptionKilogramPerKilonewtonHour,

    /// `volume_flow_acceleration_cubic_meter_per_second_squared`
    VolumeFlowAccelerationCubicMeterPerSecondSquared,
    /// `volume_flow_acceleration_liter_per_minute_per_second`
    VolumeFlowAccelerationLiterPerMinutePerSecond,
}

impl<'de> Deserialize<'de> for UnitIdEnumReaderV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // The remote derive supplies the enum mapping; reading a string first rejects object forms.
        let id = String::deserialize(deserializer)?;
        Self::deserialize(serde::de::value::StrDeserializer::new(&id))
    }
}

impl From<UnitIdEnumReaderV1> for UnitId {
    fn from(unit: UnitIdEnumReaderV1) -> Self {
        match unit {
            UnitIdEnumReaderV1::None => Self::None,
            UnitIdEnumReaderV1::LengthMeter => Self::Length_Meter,
            UnitIdEnumReaderV1::LengthPicometer => Self::Length_Picometer,
            UnitIdEnumReaderV1::LengthNanometer => Self::Length_Nanometer,
            UnitIdEnumReaderV1::LengthMicrometer => Self::Length_Micrometer,
            UnitIdEnumReaderV1::LengthMillimeter => Self::Length_Millimeter,
            UnitIdEnumReaderV1::LengthCentimeter => Self::Length_Centimeter,
            UnitIdEnumReaderV1::LengthKilometer => Self::Length_Kilometer,
            UnitIdEnumReaderV1::LengthFoot => Self::Length_Foot,
            UnitIdEnumReaderV1::LengthInch => Self::Length_Inch,
            UnitIdEnumReaderV1::LengthYard => Self::Length_Yard,
            UnitIdEnumReaderV1::LengthMile => Self::Length_Mile,
            UnitIdEnumReaderV1::MassKilogram => Self::Mass_Kilogram,
            UnitIdEnumReaderV1::MassGram => Self::Mass_Gram,
            UnitIdEnumReaderV1::MassPicogram => Self::Mass_Picogram,
            UnitIdEnumReaderV1::MassNanogram => Self::Mass_Nanogram,
            UnitIdEnumReaderV1::MassMicrogram => Self::Mass_Microgram,
            UnitIdEnumReaderV1::MassMilligram => Self::Mass_Milligram,
            UnitIdEnumReaderV1::MassMegagram => Self::Mass_Megagram,
            UnitIdEnumReaderV1::MassTonne => Self::Mass_Tonne,
            UnitIdEnumReaderV1::MassPound => Self::Mass_Pound,
            UnitIdEnumReaderV1::MassOunce => Self::Mass_Ounce,
            UnitIdEnumReaderV1::MassStone => Self::Mass_Stone,
            UnitIdEnumReaderV1::TimeSecond => Self::Time_Second,
            UnitIdEnumReaderV1::TimePicosecond => Self::Time_Picosecond,
            UnitIdEnumReaderV1::TimeNanosecond => Self::Time_Nanosecond,
            UnitIdEnumReaderV1::TimeMicrosecond => Self::Time_Microsecond,
            UnitIdEnumReaderV1::TimeMillisecond => Self::Time_Millisecond,
            UnitIdEnumReaderV1::TimeKilosecond => Self::Time_Kilosecond,
            UnitIdEnumReaderV1::TimeMinute => Self::Time_Minute,
            UnitIdEnumReaderV1::TimeHour => Self::Time_Hour,
            UnitIdEnumReaderV1::TimeDay => Self::Time_Day,
            UnitIdEnumReaderV1::TimeWeek => Self::Time_Week,
            UnitIdEnumReaderV1::TimeYear => Self::Time_Year,
            UnitIdEnumReaderV1::CurrentAmpere => Self::Current_Ampere,
            UnitIdEnumReaderV1::CurrentNanoampere => Self::Current_Nanoampere,
            UnitIdEnumReaderV1::CurrentMicroampere => Self::Current_Microampere,
            UnitIdEnumReaderV1::CurrentMilliampere => Self::Current_Milliampere,
            UnitIdEnumReaderV1::CurrentKiloampere => Self::Current_Kiloampere,
            UnitIdEnumReaderV1::CurrentMegaampere => Self::Current_Megaampere,
            UnitIdEnumReaderV1::TemperatureKelvin => Self::Temperature_Kelvin,
            UnitIdEnumReaderV1::TemperatureNanokelvin => Self::Temperature_Nanokelvin,
            UnitIdEnumReaderV1::TemperatureMicrokelvin => Self::Temperature_Microkelvin,
            UnitIdEnumReaderV1::TemperatureMillikelvin => Self::Temperature_Millikelvin,
            UnitIdEnumReaderV1::TemperatureKilokelvin => Self::Temperature_Kilokelvin,
            UnitIdEnumReaderV1::TemperatureCelsius => Self::Temperature_Celsius,
            UnitIdEnumReaderV1::TemperatureFahrenheit => Self::Temperature_Fahrenheit,
            UnitIdEnumReaderV1::AmountMole => Self::Amount_Mole,
            UnitIdEnumReaderV1::AmountPicomole => Self::Amount_Picomole,
            UnitIdEnumReaderV1::AmountNanomole => Self::Amount_Nanomole,
            UnitIdEnumReaderV1::AmountMicromole => Self::Amount_Micromole,
            UnitIdEnumReaderV1::AmountMillimole => Self::Amount_Millimole,
            UnitIdEnumReaderV1::AmountKilomole => Self::Amount_Kilomole,
            UnitIdEnumReaderV1::FrequencyHertz => Self::Frequency_Hertz,
            UnitIdEnumReaderV1::FrequencyPicohertz => Self::Frequency_Picohertz,
            UnitIdEnumReaderV1::FrequencyNanohertz => Self::Frequency_Nanohertz,
            UnitIdEnumReaderV1::FrequencyMicrohertz => Self::Frequency_Microhertz,
            UnitIdEnumReaderV1::FrequencyMillihertz => Self::Frequency_Millihertz,
            UnitIdEnumReaderV1::FrequencyKilohertz => Self::Frequency_Kilohertz,
            UnitIdEnumReaderV1::FrequencyMegahertz => Self::Frequency_Megahertz,
            UnitIdEnumReaderV1::FrequencyGigahertz => Self::Frequency_Gigahertz,
            UnitIdEnumReaderV1::FrequencyTerahertz => Self::Frequency_Terahertz,
            UnitIdEnumReaderV1::AngleRadian => Self::Angle_Radian,
            UnitIdEnumReaderV1::AnglePicoradian => Self::Angle_Picoradian,
            UnitIdEnumReaderV1::AngleNanoradian => Self::Angle_Nanoradian,
            UnitIdEnumReaderV1::AngleMicroradian => Self::Angle_Microradian,
            UnitIdEnumReaderV1::AngleMilliradian => Self::Angle_Milliradian,
            UnitIdEnumReaderV1::AngleKiloradian => Self::Angle_Kiloradian,
            UnitIdEnumReaderV1::AngleMegaradian => Self::Angle_Megaradian,
            UnitIdEnumReaderV1::AngleGigaradian => Self::Angle_Gigaradian,
            UnitIdEnumReaderV1::AngleTeraradian => Self::Angle_Teraradian,
            UnitIdEnumReaderV1::AngleDegree => Self::Angle_Degree,
            UnitIdEnumReaderV1::AngleTurn => Self::Angle_Turn,
            UnitIdEnumReaderV1::AngleArcminute => Self::Angle_Arcminute,
            UnitIdEnumReaderV1::AngleArcsecond => Self::Angle_Arcsecond,
            UnitIdEnumReaderV1::SolidAngleSteradian => Self::SolidAngle_Steradian,
            UnitIdEnumReaderV1::SolidAnglePicosteradian => Self::SolidAngle_Picosteradian,
            UnitIdEnumReaderV1::SolidAngleNanosteradian => Self::SolidAngle_Nanosteradian,
            UnitIdEnumReaderV1::SolidAngleMicrosteradian => Self::SolidAngle_Microsteradian,
            UnitIdEnumReaderV1::SolidAngleMillisteradian => Self::SolidAngle_Millisteradian,
            UnitIdEnumReaderV1::ForceNewton => Self::Force_Newton,
            UnitIdEnumReaderV1::ForcePiconewton => Self::Force_Piconewton,
            UnitIdEnumReaderV1::ForceNanonewton => Self::Force_Nanonewton,
            UnitIdEnumReaderV1::ForceMicronewton => Self::Force_Micronewton,
            UnitIdEnumReaderV1::ForceMillinewton => Self::Force_Millinewton,
            UnitIdEnumReaderV1::ForceKilonewton => Self::Force_Kilonewton,
            UnitIdEnumReaderV1::ForceMeganewton => Self::Force_Meganewton,
            UnitIdEnumReaderV1::ForceGiganewton => Self::Force_Giganewton,
            UnitIdEnumReaderV1::ForceTeranewton => Self::Force_Teranewton,
            UnitIdEnumReaderV1::ForceKilogramForce => Self::Force_KilogramForce,
            UnitIdEnumReaderV1::ForcePoundForce => Self::Force_PoundForce,
            UnitIdEnumReaderV1::PressurePascal => Self::Pressure_Pascal,
            UnitIdEnumReaderV1::PressurePicopascal => Self::Pressure_Picopascal,
            UnitIdEnumReaderV1::PressureNanopascal => Self::Pressure_Nanopascal,
            UnitIdEnumReaderV1::PressureMicropascal => Self::Pressure_Micropascal,
            UnitIdEnumReaderV1::PressureMillipascal => Self::Pressure_Millipascal,
            UnitIdEnumReaderV1::PressureKilopascal => Self::Pressure_Kilopascal,
            UnitIdEnumReaderV1::PressureMegapascal => Self::Pressure_Megapascal,
            UnitIdEnumReaderV1::PressureGigapascal => Self::Pressure_Gigapascal,
            UnitIdEnumReaderV1::PressureTerapascal => Self::Pressure_Terapascal,
            UnitIdEnumReaderV1::PressureBar => Self::Pressure_Bar,
            UnitIdEnumReaderV1::PressureMillibar => Self::Pressure_Millibar,
            UnitIdEnumReaderV1::PressureStandardAtmosphere => Self::Pressure_StandardAtmosphere,
            UnitIdEnumReaderV1::PressurePoundPerSquareInch => Self::Pressure_PoundPerSquareInch,
            UnitIdEnumReaderV1::PressureMillimeterOfMercury => Self::Pressure_MillimeterOfMercury,
            UnitIdEnumReaderV1::EnergyJoule => Self::Energy_Joule,
            UnitIdEnumReaderV1::EnergyPicojoule => Self::Energy_Picojoule,
            UnitIdEnumReaderV1::EnergyNanojoule => Self::Energy_Nanojoule,
            UnitIdEnumReaderV1::EnergyMicrojoule => Self::Energy_Microjoule,
            UnitIdEnumReaderV1::EnergyMillijoule => Self::Energy_Millijoule,
            UnitIdEnumReaderV1::EnergyKilojoule => Self::Energy_Kilojoule,
            UnitIdEnumReaderV1::EnergyMegajoule => Self::Energy_Megajoule,
            UnitIdEnumReaderV1::EnergyGigajoule => Self::Energy_Gigajoule,
            UnitIdEnumReaderV1::EnergyTerajoule => Self::Energy_Terajoule,
            UnitIdEnumReaderV1::EnergyWattHour => Self::Energy_WattHour,
            UnitIdEnumReaderV1::EnergyKilowattHour => Self::Energy_KilowattHour,
            UnitIdEnumReaderV1::EnergyElectronvolt => Self::Energy_Electronvolt,
            UnitIdEnumReaderV1::PowerWatt => Self::Power_Watt,
            UnitIdEnumReaderV1::PowerPicowatt => Self::Power_Picowatt,
            UnitIdEnumReaderV1::PowerNanowatt => Self::Power_Nanowatt,
            UnitIdEnumReaderV1::PowerMicrowatt => Self::Power_Microwatt,
            UnitIdEnumReaderV1::PowerMilliwatt => Self::Power_Milliwatt,
            UnitIdEnumReaderV1::PowerKilowatt => Self::Power_Kilowatt,
            UnitIdEnumReaderV1::PowerMegawatt => Self::Power_Megawatt,
            UnitIdEnumReaderV1::PowerGigawatt => Self::Power_Gigawatt,
            UnitIdEnumReaderV1::PowerTerawatt => Self::Power_Terawatt,
            UnitIdEnumReaderV1::PowerMechanicalHorsepower => Self::Power_MechanicalHorsepower,
            UnitIdEnumReaderV1::PowerMetricHorsepower => Self::Power_MetricHorsepower,
            UnitIdEnumReaderV1::ElectricChargeCoulomb => Self::ElectricCharge_Coulomb,
            UnitIdEnumReaderV1::ElectricChargePicocoulomb => Self::ElectricCharge_Picocoulomb,
            UnitIdEnumReaderV1::ElectricChargeNanocoulomb => Self::ElectricCharge_Nanocoulomb,
            UnitIdEnumReaderV1::ElectricChargeMicrocoulomb => Self::ElectricCharge_Microcoulomb,
            UnitIdEnumReaderV1::ElectricChargeMillicoulomb => Self::ElectricCharge_Millicoulomb,
            UnitIdEnumReaderV1::ElectricChargeKilocoulomb => Self::ElectricCharge_Kilocoulomb,
            UnitIdEnumReaderV1::ElectricChargeMegacoulomb => Self::ElectricCharge_Megacoulomb,
            UnitIdEnumReaderV1::ElectricChargeGigacoulomb => Self::ElectricCharge_Gigacoulomb,
            UnitIdEnumReaderV1::ElectricChargeTeracoulomb => Self::ElectricCharge_Teracoulomb,
            UnitIdEnumReaderV1::ElectricChargeAmpereHour => Self::ElectricCharge_AmpereHour,
            UnitIdEnumReaderV1::ElectricChargeMilliampereHour => {
                Self::ElectricCharge_MilliampereHour
            }
            UnitIdEnumReaderV1::VoltageVolt => Self::Voltage_Volt,
            UnitIdEnumReaderV1::VoltagePicovolt => Self::Voltage_Picovolt,
            UnitIdEnumReaderV1::VoltageNanovolt => Self::Voltage_Nanovolt,
            UnitIdEnumReaderV1::VoltageMicrovolt => Self::Voltage_Microvolt,
            UnitIdEnumReaderV1::VoltageMillivolt => Self::Voltage_Millivolt,
            UnitIdEnumReaderV1::VoltageKilovolt => Self::Voltage_Kilovolt,
            UnitIdEnumReaderV1::VoltageMegavolt => Self::Voltage_Megavolt,
            UnitIdEnumReaderV1::VoltageGigavolt => Self::Voltage_Gigavolt,
            UnitIdEnumReaderV1::VoltageTeravolt => Self::Voltage_Teravolt,
            UnitIdEnumReaderV1::ResistanceOhm => Self::Resistance_Ohm,
            UnitIdEnumReaderV1::ResistancePicoohm => Self::Resistance_Picoohm,
            UnitIdEnumReaderV1::ResistanceNanoohm => Self::Resistance_Nanoohm,
            UnitIdEnumReaderV1::ResistanceMicroohm => Self::Resistance_Microohm,
            UnitIdEnumReaderV1::ResistanceMilliohm => Self::Resistance_Milliohm,
            UnitIdEnumReaderV1::ResistanceKiloohm => Self::Resistance_Kiloohm,
            UnitIdEnumReaderV1::ResistanceMegaohm => Self::Resistance_Megaohm,
            UnitIdEnumReaderV1::ResistanceGigaohm => Self::Resistance_Gigaohm,
            UnitIdEnumReaderV1::ResistanceTeraohm => Self::Resistance_Teraohm,
            UnitIdEnumReaderV1::ConductanceSiemens => Self::Conductance_Siemens,
            UnitIdEnumReaderV1::ConductancePicosiemens => Self::Conductance_Picosiemens,
            UnitIdEnumReaderV1::ConductanceNanosiemens => Self::Conductance_Nanosiemens,
            UnitIdEnumReaderV1::ConductanceMicrosiemens => Self::Conductance_Microsiemens,
            UnitIdEnumReaderV1::ConductanceMillisiemens => Self::Conductance_Millisiemens,
            UnitIdEnumReaderV1::ConductanceKilosiemens => Self::Conductance_Kilosiemens,
            UnitIdEnumReaderV1::ConductanceMegasiemens => Self::Conductance_Megasiemens,
            UnitIdEnumReaderV1::ConductanceGigasiemens => Self::Conductance_Gigasiemens,
            UnitIdEnumReaderV1::ConductanceTerasiemens => Self::Conductance_Terasiemens,
            UnitIdEnumReaderV1::CapacitanceFarad => Self::Capacitance_Farad,
            UnitIdEnumReaderV1::CapacitancePicofarad => Self::Capacitance_Picofarad,
            UnitIdEnumReaderV1::CapacitanceNanofarad => Self::Capacitance_Nanofarad,
            UnitIdEnumReaderV1::CapacitanceMicrofarad => Self::Capacitance_Microfarad,
            UnitIdEnumReaderV1::CapacitanceMillifarad => Self::Capacitance_Millifarad,
            UnitIdEnumReaderV1::CapacitanceKilofarad => Self::Capacitance_Kilofarad,
            UnitIdEnumReaderV1::InductanceHenry => Self::Inductance_Henry,
            UnitIdEnumReaderV1::InductancePicohenry => Self::Inductance_Picohenry,
            UnitIdEnumReaderV1::InductanceNanohenry => Self::Inductance_Nanohenry,
            UnitIdEnumReaderV1::InductanceMicrohenry => Self::Inductance_Microhenry,
            UnitIdEnumReaderV1::InductanceMillihenry => Self::Inductance_Millihenry,
            UnitIdEnumReaderV1::MagneticFluxDensityTesla => Self::MagneticFluxDensity_Tesla,
            UnitIdEnumReaderV1::MagneticFluxDensityPicotesla => Self::MagneticFluxDensity_Picotesla,
            UnitIdEnumReaderV1::MagneticFluxDensityNanotesla => Self::MagneticFluxDensity_Nanotesla,
            UnitIdEnumReaderV1::MagneticFluxDensityMicrotesla => {
                Self::MagneticFluxDensity_Microtesla
            }
            UnitIdEnumReaderV1::MagneticFluxDensityMillitesla => {
                Self::MagneticFluxDensity_Millitesla
            }
            UnitIdEnumReaderV1::MagneticFluxWeber => Self::MagneticFlux_Weber,
            UnitIdEnumReaderV1::MagneticFluxPicoweber => Self::MagneticFlux_Picoweber,
            UnitIdEnumReaderV1::MagneticFluxNanoweber => Self::MagneticFlux_Nanoweber,
            UnitIdEnumReaderV1::MagneticFluxMicroweber => Self::MagneticFlux_Microweber,
            UnitIdEnumReaderV1::MagneticFluxMilliweber => Self::MagneticFlux_Milliweber,
            UnitIdEnumReaderV1::MagneticFluxKiloweber => Self::MagneticFlux_Kiloweber,
            UnitIdEnumReaderV1::MagneticFluxMegaweber => Self::MagneticFlux_Megaweber,
            UnitIdEnumReaderV1::MagneticFluxGigaweber => Self::MagneticFlux_Gigaweber,
            UnitIdEnumReaderV1::MagneticFluxTeraweber => Self::MagneticFlux_Teraweber,
            UnitIdEnumReaderV1::AreaSquareMeter => Self::Area_SquareMeter,
            UnitIdEnumReaderV1::AreaSquareMicrometer => Self::Area_SquareMicrometer,
            UnitIdEnumReaderV1::AreaSquareMillimeter => Self::Area_SquareMillimeter,
            UnitIdEnumReaderV1::AreaSquareCentimeter => Self::Area_SquareCentimeter,
            UnitIdEnumReaderV1::AreaSquareKilometer => Self::Area_SquareKilometer,
            UnitIdEnumReaderV1::AreaSquareFoot => Self::Area_SquareFoot,
            UnitIdEnumReaderV1::AreaSquareInch => Self::Area_SquareInch,
            UnitIdEnumReaderV1::AreaAcre => Self::Area_Acre,
            UnitIdEnumReaderV1::AreaHectare => Self::Area_Hectare,
            UnitIdEnumReaderV1::AreaSquareMile => Self::Area_SquareMile,
            UnitIdEnumReaderV1::VolumeCubicMeter => Self::Volume_CubicMeter,
            UnitIdEnumReaderV1::VolumeCubicCentimeter => Self::Volume_CubicCentimeter,
            UnitIdEnumReaderV1::VolumeCubicMillimeter => Self::Volume_CubicMillimeter,
            UnitIdEnumReaderV1::VolumeLiter => Self::Volume_Liter,
            UnitIdEnumReaderV1::VolumePicoliter => Self::Volume_Picoliter,
            UnitIdEnumReaderV1::VolumeNanoliter => Self::Volume_Nanoliter,
            UnitIdEnumReaderV1::VolumeMicroliter => Self::Volume_Microliter,
            UnitIdEnumReaderV1::VolumeMilliliter => Self::Volume_Milliliter,
            UnitIdEnumReaderV1::VolumeKiloliter => Self::Volume_Kiloliter,
            UnitIdEnumReaderV1::VolumeMegaliter => Self::Volume_Megaliter,
            UnitIdEnumReaderV1::VolumeGallon => Self::Volume_Gallon,
            UnitIdEnumReaderV1::VolumeImperialGallon => Self::Volume_ImperialGallon,
            UnitIdEnumReaderV1::VolumeFluidOunce => Self::Volume_FluidOunce,
            UnitIdEnumReaderV1::VolumeImperialFluidOunce => Self::Volume_ImperialFluidOunce,
            UnitIdEnumReaderV1::VolumeCup => Self::Volume_Cup,
            UnitIdEnumReaderV1::VolumePint => Self::Volume_Pint,
            UnitIdEnumReaderV1::VolumeQuart => Self::Volume_Quart,
            UnitIdEnumReaderV1::SpeedMeterPerSecond => Self::Speed_MeterPerSecond,
            UnitIdEnumReaderV1::SpeedMillimeterPerSecond => Self::Speed_MillimeterPerSecond,
            UnitIdEnumReaderV1::SpeedCentimeterPerSecond => Self::Speed_CentimeterPerSecond,
            UnitIdEnumReaderV1::SpeedKilometerPerHour => Self::Speed_KilometerPerHour,
            UnitIdEnumReaderV1::SpeedInchPerSecond => Self::Speed_InchPerSecond,
            UnitIdEnumReaderV1::SpeedFootPerSecond => Self::Speed_FootPerSecond,
            UnitIdEnumReaderV1::SpeedMilePerHour => Self::Speed_MilePerHour,
            UnitIdEnumReaderV1::SpeedKnot => Self::Speed_Knot,
            UnitIdEnumReaderV1::AccelerationMeterPerSecondSquared => {
                Self::Acceleration_MeterPerSecondSquared
            }
            UnitIdEnumReaderV1::AccelerationMillimeterPerSecondSquared => {
                Self::Acceleration_MillimeterPerSecondSquared
            }
            UnitIdEnumReaderV1::AccelerationCentimeterPerSecondSquared => {
                Self::Acceleration_CentimeterPerSecondSquared
            }
            UnitIdEnumReaderV1::AccelerationInchPerSecondSquared => {
                Self::Acceleration_InchPerSecondSquared
            }
            UnitIdEnumReaderV1::AccelerationFootPerSecondSquared => {
                Self::Acceleration_FootPerSecondSquared
            }
            UnitIdEnumReaderV1::AccelerationStandardGravity => Self::Acceleration_StandardGravity,
            UnitIdEnumReaderV1::AngularVelocityRadianPerSecond => {
                Self::AngularVelocity_RadianPerSecond
            }
            UnitIdEnumReaderV1::AngularVelocityDegreePerSecond => {
                Self::AngularVelocity_DegreePerSecond
            }
            UnitIdEnumReaderV1::AngularVelocityRevolutionPerMinute => {
                Self::AngularVelocity_RevolutionPerMinute
            }
            UnitIdEnumReaderV1::AngularVelocityRevolutionPerSecond => {
                Self::AngularVelocity_RevolutionPerSecond
            }
            UnitIdEnumReaderV1::TorqueNewtonMeter => Self::Torque_NewtonMeter,
            UnitIdEnumReaderV1::TorqueNewtonMillimeter => Self::Torque_NewtonMillimeter,
            UnitIdEnumReaderV1::TorqueNewtonCentimeter => Self::Torque_NewtonCentimeter,
            UnitIdEnumReaderV1::TorquePoundForceInch => Self::Torque_PoundForceInch,
            UnitIdEnumReaderV1::TorquePoundForceFoot => Self::Torque_PoundForceFoot,
            UnitIdEnumReaderV1::DensityKilogramPerCubicMeter => Self::Density_KilogramPerCubicMeter,
            UnitIdEnumReaderV1::DensityGramPerCubicCentimeter => {
                Self::Density_GramPerCubicCentimeter
            }
            UnitIdEnumReaderV1::DensityGramPerLiter => Self::Density_GramPerLiter,
            UnitIdEnumReaderV1::DensityKilogramPerLiter => Self::Density_KilogramPerLiter,
            UnitIdEnumReaderV1::DensityPoundPerCubicFoot => Self::Density_PoundPerCubicFoot,
            UnitIdEnumReaderV1::VolumeFlowRateCubicMeterPerSecond => {
                Self::VolumeFlowRate_CubicMeterPerSecond
            }
            UnitIdEnumReaderV1::VolumeFlowRateLiterPerMinute => Self::VolumeFlowRate_LiterPerMinute,
            UnitIdEnumReaderV1::DisplacementPerRevolutionCubicMeterPerRevolution => {
                Self::DisplacementPerRevolution_CubicMeterPerRevolution
            }
            UnitIdEnumReaderV1::DisplacementPerRevolutionCubicCentimeterPerRevolution => {
                Self::DisplacementPerRevolution_CubicCentimeterPerRevolution
            }
            UnitIdEnumReaderV1::AngularFrequencyRadianPerSecond => {
                Self::AngularFrequency_RadianPerSecond
            }
            UnitIdEnumReaderV1::AngularFrequencyPicoradianPerSecond => {
                Self::AngularFrequency_PicoradianPerSecond
            }
            UnitIdEnumReaderV1::AngularFrequencyNanoradianPerSecond => {
                Self::AngularFrequency_NanoradianPerSecond
            }
            UnitIdEnumReaderV1::AngularFrequencyMicroradianPerSecond => {
                Self::AngularFrequency_MicroradianPerSecond
            }
            UnitIdEnumReaderV1::AngularFrequencyMilliradianPerSecond => {
                Self::AngularFrequency_MilliradianPerSecond
            }
            UnitIdEnumReaderV1::AngularFrequencyKiloradianPerSecond => {
                Self::AngularFrequency_KiloradianPerSecond
            }
            UnitIdEnumReaderV1::AngularFrequencyMegaradianPerSecond => {
                Self::AngularFrequency_MegaradianPerSecond
            }
            UnitIdEnumReaderV1::AngularFrequencyGigaradianPerSecond => {
                Self::AngularFrequency_GigaradianPerSecond
            }
            UnitIdEnumReaderV1::AngularFrequencyTeraradianPerSecond => {
                Self::AngularFrequency_TeraradianPerSecond
            }
            UnitIdEnumReaderV1::MomentOfInertiaKilogramSquareMeter => {
                Self::MomentOfInertia_KilogramSquareMeter
            }
            UnitIdEnumReaderV1::MomentOfInertiaKilogramSquareCentimeter => {
                Self::MomentOfInertia_KilogramSquareCentimeter
            }
            UnitIdEnumReaderV1::MomentOfInertiaGramSquareCentimeter => {
                Self::MomentOfInertia_GramSquareCentimeter
            }
            UnitIdEnumReaderV1::LinearStiffnessNewtonPerMeter => {
                Self::LinearStiffness_NewtonPerMeter
            }
            UnitIdEnumReaderV1::LinearStiffnessNewtonPerMillimeter => {
                Self::LinearStiffness_NewtonPerMillimeter
            }
            UnitIdEnumReaderV1::LinearStiffnessKilonewtonPerMeter => {
                Self::LinearStiffness_KilonewtonPerMeter
            }
            UnitIdEnumReaderV1::LinearDampingNewtonSecondPerMeter => {
                Self::LinearDamping_NewtonSecondPerMeter
            }
            UnitIdEnumReaderV1::LinearDampingNewtonSecondPerMillimeter => {
                Self::LinearDamping_NewtonSecondPerMillimeter
            }
            UnitIdEnumReaderV1::LinearDampingKilonewtonSecondPerMeter => {
                Self::LinearDamping_KilonewtonSecondPerMeter
            }
            UnitIdEnumReaderV1::RotationalStiffnessNewtonMeterPerRadian => {
                Self::RotationalStiffness_NewtonMeterPerRadian
            }
            UnitIdEnumReaderV1::RotationalStiffnessNewtonMillimeterPerRadian => {
                Self::RotationalStiffness_NewtonMillimeterPerRadian
            }
            UnitIdEnumReaderV1::RotationalStiffnessKilonewtonMeterPerRadian => {
                Self::RotationalStiffness_KilonewtonMeterPerRadian
            }
            UnitIdEnumReaderV1::RotationalDampingNewtonMeterSecondPerRadian => {
                Self::RotationalDamping_NewtonMeterSecondPerRadian
            }
            UnitIdEnumReaderV1::RotationalDampingNewtonMillimeterSecondPerRadian => {
                Self::RotationalDamping_NewtonMillimeterSecondPerRadian
            }
            UnitIdEnumReaderV1::RotationalDampingKilonewtonMeterSecondPerRadian => {
                Self::RotationalDamping_KilonewtonMeterSecondPerRadian
            }
            UnitIdEnumReaderV1::DynamicViscosityPascalSecond => Self::DynamicViscosity_PascalSecond,
            UnitIdEnumReaderV1::DynamicViscosityMillipascalSecond => {
                Self::DynamicViscosity_MillipascalSecond
            }
            UnitIdEnumReaderV1::DynamicViscosityPoise => Self::DynamicViscosity_Poise,
            UnitIdEnumReaderV1::DynamicViscosityCentipoise => Self::DynamicViscosity_Centipoise,
            UnitIdEnumReaderV1::KinematicViscositySquareMeterPerSecond => {
                Self::KinematicViscosity_SquareMeterPerSecond
            }
            UnitIdEnumReaderV1::KinematicViscositySquareMillimeterPerSecond => {
                Self::KinematicViscosity_SquareMillimeterPerSecond
            }
            UnitIdEnumReaderV1::KinematicViscosityStokes => Self::KinematicViscosity_Stokes,
            UnitIdEnumReaderV1::KinematicViscosityCentistokes => {
                Self::KinematicViscosity_Centistokes
            }
            UnitIdEnumReaderV1::MassFlowRateKilogramPerSecond => {
                Self::MassFlowRate_KilogramPerSecond
            }
            UnitIdEnumReaderV1::MassFlowRateGramPerSecond => Self::MassFlowRate_GramPerSecond,
            UnitIdEnumReaderV1::MassFlowRateKilogramPerMinute => {
                Self::MassFlowRate_KilogramPerMinute
            }
            UnitIdEnumReaderV1::MassFlowRateKilogramPerHour => Self::MassFlowRate_KilogramPerHour,
            UnitIdEnumReaderV1::MomentumKilogramMeterPerSecond => {
                Self::Momentum_KilogramMeterPerSecond
            }
            UnitIdEnumReaderV1::MomentumGramCentimeterPerSecond => {
                Self::Momentum_GramCentimeterPerSecond
            }
            UnitIdEnumReaderV1::MomentumNewtonSecond => Self::Momentum_NewtonSecond,
            UnitIdEnumReaderV1::HydraulicLeakageCoefficientCubicMeterPerSecondPerPascal => {
                Self::HydraulicLeakageCoefficient_CubicMeterPerSecondPerPascal
            }
            UnitIdEnumReaderV1::HydraulicLeakageCoefficientLiterPerMinutePerBar => {
                Self::HydraulicLeakageCoefficient_LiterPerMinutePerBar
            }
            UnitIdEnumReaderV1::SpecificHeatCapacityJoulePerKilogramKelvin => {
                Self::SpecificHeatCapacity_JoulePerKilogramKelvin
            }
            UnitIdEnumReaderV1::SpecificHeatCapacityKilojoulePerKilogramKelvin => {
                Self::SpecificHeatCapacity_KilojoulePerKilogramKelvin
            }
            UnitIdEnumReaderV1::SpecificGasConstantJoulePerKilogramKelvin => {
                Self::SpecificGasConstant_JoulePerKilogramKelvin
            }
            UnitIdEnumReaderV1::SpecificGasConstantKilojoulePerKilogramKelvin => {
                Self::SpecificGasConstant_KilojoulePerKilogramKelvin
            }
            UnitIdEnumReaderV1::ThermalConductanceWattPerKelvin => {
                Self::ThermalConductance_WattPerKelvin
            }
            UnitIdEnumReaderV1::ThermalConductanceMilliwattPerKelvin => {
                Self::ThermalConductance_MilliwattPerKelvin
            }
            UnitIdEnumReaderV1::ThermalConductanceKilowattPerKelvin => {
                Self::ThermalConductance_KilowattPerKelvin
            }
            UnitIdEnumReaderV1::HydraulicResistancePascalSecondPerCubicMeter => {
                Self::HydraulicResistance_PascalSecondPerCubicMeter
            }
            UnitIdEnumReaderV1::HydraulicResistanceBarMinutePerLiter => {
                Self::HydraulicResistance_BarMinutePerLiter
            }
            UnitIdEnumReaderV1::PneumaticCharacteristicImpedancePascalSecondPerJoule => {
                Self::PneumaticCharacteristicImpedance_PascalSecondPerJoule
            }
            UnitIdEnumReaderV1::PneumaticCharacteristicImpedanceSecondPerCubicMeter => {
                Self::PneumaticCharacteristicImpedance_SecondPerCubicMeter
            }
            UnitIdEnumReaderV1::TurbulentFlowCoefficientCubicMeterPerSecondPerSquareRootPascal => {
                Self::TurbulentFlowCoefficient_CubicMeterPerSecondPerSquareRootPascal
            }
            UnitIdEnumReaderV1::TurbulentFlowCoefficientLiterPerMinutePerSquareRootBar => {
                Self::TurbulentFlowCoefficient_LiterPerMinutePerSquareRootBar
            }
            UnitIdEnumReaderV1::MotorBackEmfConstantVoltSecondPerRadian => {
                Self::MotorBackEmfConstant_VoltSecondPerRadian
            }
            UnitIdEnumReaderV1::MotorBackEmfConstantMillivoltSecondPerRadian => {
                Self::MotorBackEmfConstant_MillivoltSecondPerRadian
            }
            UnitIdEnumReaderV1::MotorBackEmfConstantVoltPerRevolutionPerMinute => {
                Self::MotorBackEmfConstant_VoltPerRevolutionPerMinute
            }
            UnitIdEnumReaderV1::ScrewPitchMeterPerRadian => Self::ScrewPitch_MeterPerRadian,
            UnitIdEnumReaderV1::ScrewPitchMillimeterPerRadian => {
                Self::ScrewPitch_MillimeterPerRadian
            }
            UnitIdEnumReaderV1::ScrewPitchMillimeterPerRevolution => {
                Self::ScrewPitch_MillimeterPerRevolution
            }
            UnitIdEnumReaderV1::ThrustSpecificFuelConsumptionKilogramPerNewtonSecond => {
                Self::ThrustSpecificFuelConsumption_KilogramPerNewtonSecond
            }
            UnitIdEnumReaderV1::ThrustSpecificFuelConsumptionKilogramPerKilonewtonHour => {
                Self::ThrustSpecificFuelConsumption_KilogramPerKilonewtonHour
            }
            UnitIdEnumReaderV1::VolumeFlowAccelerationCubicMeterPerSecondSquared => {
                Self::VolumeFlowAcceleration_CubicMeterPerSecondSquared
            }
            UnitIdEnumReaderV1::VolumeFlowAccelerationLiterPerMinutePerSecond => {
                Self::VolumeFlowAcceleration_LiterPerMinutePerSecond
            }
        }
    }
}

/// Every frozen identifier and its expected current unit.
#[cfg(test)]
const UNIT_IDS_V1: &[(&str, UnitId)] = &[
    ("none", UnitId::None),
    ("length_meter", UnitId::Length_Meter),
    ("length_picometer", UnitId::Length_Picometer),
    ("length_nanometer", UnitId::Length_Nanometer),
    ("length_micrometer", UnitId::Length_Micrometer),
    ("length_millimeter", UnitId::Length_Millimeter),
    ("length_centimeter", UnitId::Length_Centimeter),
    ("length_kilometer", UnitId::Length_Kilometer),
    ("length_foot", UnitId::Length_Foot),
    ("length_inch", UnitId::Length_Inch),
    ("length_yard", UnitId::Length_Yard),
    ("length_mile", UnitId::Length_Mile),
    ("mass_kilogram", UnitId::Mass_Kilogram),
    ("mass_gram", UnitId::Mass_Gram),
    ("mass_picogram", UnitId::Mass_Picogram),
    ("mass_nanogram", UnitId::Mass_Nanogram),
    ("mass_microgram", UnitId::Mass_Microgram),
    ("mass_milligram", UnitId::Mass_Milligram),
    ("mass_megagram", UnitId::Mass_Megagram),
    ("mass_tonne", UnitId::Mass_Tonne),
    ("mass_pound", UnitId::Mass_Pound),
    ("mass_ounce", UnitId::Mass_Ounce),
    ("mass_stone", UnitId::Mass_Stone),
    ("time_second", UnitId::Time_Second),
    ("time_picosecond", UnitId::Time_Picosecond),
    ("time_nanosecond", UnitId::Time_Nanosecond),
    ("time_microsecond", UnitId::Time_Microsecond),
    ("time_millisecond", UnitId::Time_Millisecond),
    ("time_kilosecond", UnitId::Time_Kilosecond),
    ("time_minute", UnitId::Time_Minute),
    ("time_hour", UnitId::Time_Hour),
    ("time_day", UnitId::Time_Day),
    ("time_week", UnitId::Time_Week),
    ("time_year", UnitId::Time_Year),
    ("current_ampere", UnitId::Current_Ampere),
    ("current_nanoampere", UnitId::Current_Nanoampere),
    ("current_microampere", UnitId::Current_Microampere),
    ("current_milliampere", UnitId::Current_Milliampere),
    ("current_kiloampere", UnitId::Current_Kiloampere),
    ("current_megaampere", UnitId::Current_Megaampere),
    ("temperature_kelvin", UnitId::Temperature_Kelvin),
    ("temperature_nanokelvin", UnitId::Temperature_Nanokelvin),
    ("temperature_microkelvin", UnitId::Temperature_Microkelvin),
    ("temperature_millikelvin", UnitId::Temperature_Millikelvin),
    ("temperature_kilokelvin", UnitId::Temperature_Kilokelvin),
    ("temperature_celsius", UnitId::Temperature_Celsius),
    ("temperature_fahrenheit", UnitId::Temperature_Fahrenheit),
    ("amount_mole", UnitId::Amount_Mole),
    ("amount_picomole", UnitId::Amount_Picomole),
    ("amount_nanomole", UnitId::Amount_Nanomole),
    ("amount_micromole", UnitId::Amount_Micromole),
    ("amount_millimole", UnitId::Amount_Millimole),
    ("amount_kilomole", UnitId::Amount_Kilomole),
    ("frequency_hertz", UnitId::Frequency_Hertz),
    ("frequency_picohertz", UnitId::Frequency_Picohertz),
    ("frequency_nanohertz", UnitId::Frequency_Nanohertz),
    ("frequency_microhertz", UnitId::Frequency_Microhertz),
    ("frequency_millihertz", UnitId::Frequency_Millihertz),
    ("frequency_kilohertz", UnitId::Frequency_Kilohertz),
    ("frequency_megahertz", UnitId::Frequency_Megahertz),
    ("frequency_gigahertz", UnitId::Frequency_Gigahertz),
    ("frequency_terahertz", UnitId::Frequency_Terahertz),
    ("angle_radian", UnitId::Angle_Radian),
    ("angle_picoradian", UnitId::Angle_Picoradian),
    ("angle_nanoradian", UnitId::Angle_Nanoradian),
    ("angle_microradian", UnitId::Angle_Microradian),
    ("angle_milliradian", UnitId::Angle_Milliradian),
    ("angle_kiloradian", UnitId::Angle_Kiloradian),
    ("angle_megaradian", UnitId::Angle_Megaradian),
    ("angle_gigaradian", UnitId::Angle_Gigaradian),
    ("angle_teraradian", UnitId::Angle_Teraradian),
    ("angle_degree", UnitId::Angle_Degree),
    ("angle_turn", UnitId::Angle_Turn),
    ("angle_arcminute", UnitId::Angle_Arcminute),
    ("angle_arcsecond", UnitId::Angle_Arcsecond),
    ("solid_angle_steradian", UnitId::SolidAngle_Steradian),
    (
        "solid_angle_picosteradian",
        UnitId::SolidAngle_Picosteradian,
    ),
    (
        "solid_angle_nanosteradian",
        UnitId::SolidAngle_Nanosteradian,
    ),
    (
        "solid_angle_microsteradian",
        UnitId::SolidAngle_Microsteradian,
    ),
    (
        "solid_angle_millisteradian",
        UnitId::SolidAngle_Millisteradian,
    ),
    ("force_newton", UnitId::Force_Newton),
    ("force_piconewton", UnitId::Force_Piconewton),
    ("force_nanonewton", UnitId::Force_Nanonewton),
    ("force_micronewton", UnitId::Force_Micronewton),
    ("force_millinewton", UnitId::Force_Millinewton),
    ("force_kilonewton", UnitId::Force_Kilonewton),
    ("force_meganewton", UnitId::Force_Meganewton),
    ("force_giganewton", UnitId::Force_Giganewton),
    ("force_teranewton", UnitId::Force_Teranewton),
    ("force_kilogram_force", UnitId::Force_KilogramForce),
    ("force_pound_force", UnitId::Force_PoundForce),
    ("pressure_pascal", UnitId::Pressure_Pascal),
    ("pressure_picopascal", UnitId::Pressure_Picopascal),
    ("pressure_nanopascal", UnitId::Pressure_Nanopascal),
    ("pressure_micropascal", UnitId::Pressure_Micropascal),
    ("pressure_millipascal", UnitId::Pressure_Millipascal),
    ("pressure_kilopascal", UnitId::Pressure_Kilopascal),
    ("pressure_megapascal", UnitId::Pressure_Megapascal),
    ("pressure_gigapascal", UnitId::Pressure_Gigapascal),
    ("pressure_terapascal", UnitId::Pressure_Terapascal),
    ("pressure_bar", UnitId::Pressure_Bar),
    ("pressure_millibar", UnitId::Pressure_Millibar),
    (
        "pressure_standard_atmosphere",
        UnitId::Pressure_StandardAtmosphere,
    ),
    (
        "pressure_pound_per_square_inch",
        UnitId::Pressure_PoundPerSquareInch,
    ),
    (
        "pressure_millimeter_of_mercury",
        UnitId::Pressure_MillimeterOfMercury,
    ),
    ("energy_joule", UnitId::Energy_Joule),
    ("energy_picojoule", UnitId::Energy_Picojoule),
    ("energy_nanojoule", UnitId::Energy_Nanojoule),
    ("energy_microjoule", UnitId::Energy_Microjoule),
    ("energy_millijoule", UnitId::Energy_Millijoule),
    ("energy_kilojoule", UnitId::Energy_Kilojoule),
    ("energy_megajoule", UnitId::Energy_Megajoule),
    ("energy_gigajoule", UnitId::Energy_Gigajoule),
    ("energy_terajoule", UnitId::Energy_Terajoule),
    ("energy_watt_hour", UnitId::Energy_WattHour),
    ("energy_kilowatt_hour", UnitId::Energy_KilowattHour),
    ("energy_electronvolt", UnitId::Energy_Electronvolt),
    ("power_watt", UnitId::Power_Watt),
    ("power_picowatt", UnitId::Power_Picowatt),
    ("power_nanowatt", UnitId::Power_Nanowatt),
    ("power_microwatt", UnitId::Power_Microwatt),
    ("power_milliwatt", UnitId::Power_Milliwatt),
    ("power_kilowatt", UnitId::Power_Kilowatt),
    ("power_megawatt", UnitId::Power_Megawatt),
    ("power_gigawatt", UnitId::Power_Gigawatt),
    ("power_terawatt", UnitId::Power_Terawatt),
    (
        "power_mechanical_horsepower",
        UnitId::Power_MechanicalHorsepower,
    ),
    ("power_metric_horsepower", UnitId::Power_MetricHorsepower),
    ("electric_charge_coulomb", UnitId::ElectricCharge_Coulomb),
    (
        "electric_charge_picocoulomb",
        UnitId::ElectricCharge_Picocoulomb,
    ),
    (
        "electric_charge_nanocoulomb",
        UnitId::ElectricCharge_Nanocoulomb,
    ),
    (
        "electric_charge_microcoulomb",
        UnitId::ElectricCharge_Microcoulomb,
    ),
    (
        "electric_charge_millicoulomb",
        UnitId::ElectricCharge_Millicoulomb,
    ),
    (
        "electric_charge_kilocoulomb",
        UnitId::ElectricCharge_Kilocoulomb,
    ),
    (
        "electric_charge_megacoulomb",
        UnitId::ElectricCharge_Megacoulomb,
    ),
    (
        "electric_charge_gigacoulomb",
        UnitId::ElectricCharge_Gigacoulomb,
    ),
    (
        "electric_charge_teracoulomb",
        UnitId::ElectricCharge_Teracoulomb,
    ),
    (
        "electric_charge_ampere_hour",
        UnitId::ElectricCharge_AmpereHour,
    ),
    (
        "electric_charge_milliampere_hour",
        UnitId::ElectricCharge_MilliampereHour,
    ),
    ("voltage_volt", UnitId::Voltage_Volt),
    ("voltage_picovolt", UnitId::Voltage_Picovolt),
    ("voltage_nanovolt", UnitId::Voltage_Nanovolt),
    ("voltage_microvolt", UnitId::Voltage_Microvolt),
    ("voltage_millivolt", UnitId::Voltage_Millivolt),
    ("voltage_kilovolt", UnitId::Voltage_Kilovolt),
    ("voltage_megavolt", UnitId::Voltage_Megavolt),
    ("voltage_gigavolt", UnitId::Voltage_Gigavolt),
    ("voltage_teravolt", UnitId::Voltage_Teravolt),
    ("resistance_ohm", UnitId::Resistance_Ohm),
    ("resistance_picoohm", UnitId::Resistance_Picoohm),
    ("resistance_nanoohm", UnitId::Resistance_Nanoohm),
    ("resistance_microohm", UnitId::Resistance_Microohm),
    ("resistance_milliohm", UnitId::Resistance_Milliohm),
    ("resistance_kiloohm", UnitId::Resistance_Kiloohm),
    ("resistance_megaohm", UnitId::Resistance_Megaohm),
    ("resistance_gigaohm", UnitId::Resistance_Gigaohm),
    ("resistance_teraohm", UnitId::Resistance_Teraohm),
    ("conductance_siemens", UnitId::Conductance_Siemens),
    ("conductance_picosiemens", UnitId::Conductance_Picosiemens),
    ("conductance_nanosiemens", UnitId::Conductance_Nanosiemens),
    ("conductance_microsiemens", UnitId::Conductance_Microsiemens),
    ("conductance_millisiemens", UnitId::Conductance_Millisiemens),
    ("conductance_kilosiemens", UnitId::Conductance_Kilosiemens),
    ("conductance_megasiemens", UnitId::Conductance_Megasiemens),
    ("conductance_gigasiemens", UnitId::Conductance_Gigasiemens),
    ("conductance_terasiemens", UnitId::Conductance_Terasiemens),
    ("capacitance_farad", UnitId::Capacitance_Farad),
    ("capacitance_picofarad", UnitId::Capacitance_Picofarad),
    ("capacitance_nanofarad", UnitId::Capacitance_Nanofarad),
    ("capacitance_microfarad", UnitId::Capacitance_Microfarad),
    ("capacitance_millifarad", UnitId::Capacitance_Millifarad),
    ("capacitance_kilofarad", UnitId::Capacitance_Kilofarad),
    ("inductance_henry", UnitId::Inductance_Henry),
    ("inductance_picohenry", UnitId::Inductance_Picohenry),
    ("inductance_nanohenry", UnitId::Inductance_Nanohenry),
    ("inductance_microhenry", UnitId::Inductance_Microhenry),
    ("inductance_millihenry", UnitId::Inductance_Millihenry),
    (
        "magnetic_flux_density_tesla",
        UnitId::MagneticFluxDensity_Tesla,
    ),
    (
        "magnetic_flux_density_picotesla",
        UnitId::MagneticFluxDensity_Picotesla,
    ),
    (
        "magnetic_flux_density_nanotesla",
        UnitId::MagneticFluxDensity_Nanotesla,
    ),
    (
        "magnetic_flux_density_microtesla",
        UnitId::MagneticFluxDensity_Microtesla,
    ),
    (
        "magnetic_flux_density_millitesla",
        UnitId::MagneticFluxDensity_Millitesla,
    ),
    ("magnetic_flux_weber", UnitId::MagneticFlux_Weber),
    ("magnetic_flux_picoweber", UnitId::MagneticFlux_Picoweber),
    ("magnetic_flux_nanoweber", UnitId::MagneticFlux_Nanoweber),
    ("magnetic_flux_microweber", UnitId::MagneticFlux_Microweber),
    ("magnetic_flux_milliweber", UnitId::MagneticFlux_Milliweber),
    ("magnetic_flux_kiloweber", UnitId::MagneticFlux_Kiloweber),
    ("magnetic_flux_megaweber", UnitId::MagneticFlux_Megaweber),
    ("magnetic_flux_gigaweber", UnitId::MagneticFlux_Gigaweber),
    ("magnetic_flux_teraweber", UnitId::MagneticFlux_Teraweber),
    ("area_square_meter", UnitId::Area_SquareMeter),
    ("area_square_micrometer", UnitId::Area_SquareMicrometer),
    ("area_square_millimeter", UnitId::Area_SquareMillimeter),
    ("area_square_centimeter", UnitId::Area_SquareCentimeter),
    ("area_square_kilometer", UnitId::Area_SquareKilometer),
    ("area_square_foot", UnitId::Area_SquareFoot),
    ("area_square_inch", UnitId::Area_SquareInch),
    ("area_acre", UnitId::Area_Acre),
    ("area_hectare", UnitId::Area_Hectare),
    ("area_square_mile", UnitId::Area_SquareMile),
    ("volume_cubic_meter", UnitId::Volume_CubicMeter),
    ("volume_cubic_centimeter", UnitId::Volume_CubicCentimeter),
    ("volume_cubic_millimeter", UnitId::Volume_CubicMillimeter),
    ("volume_liter", UnitId::Volume_Liter),
    ("volume_picoliter", UnitId::Volume_Picoliter),
    ("volume_nanoliter", UnitId::Volume_Nanoliter),
    ("volume_microliter", UnitId::Volume_Microliter),
    ("volume_milliliter", UnitId::Volume_Milliliter),
    ("volume_kiloliter", UnitId::Volume_Kiloliter),
    ("volume_megaliter", UnitId::Volume_Megaliter),
    ("volume_gallon", UnitId::Volume_Gallon),
    ("volume_imperial_gallon", UnitId::Volume_ImperialGallon),
    ("volume_fluid_ounce", UnitId::Volume_FluidOunce),
    (
        "volume_imperial_fluid_ounce",
        UnitId::Volume_ImperialFluidOunce,
    ),
    ("volume_cup", UnitId::Volume_Cup),
    ("volume_pint", UnitId::Volume_Pint),
    ("volume_quart", UnitId::Volume_Quart),
    ("speed_meter_per_second", UnitId::Speed_MeterPerSecond),
    (
        "speed_millimeter_per_second",
        UnitId::Speed_MillimeterPerSecond,
    ),
    (
        "speed_centimeter_per_second",
        UnitId::Speed_CentimeterPerSecond,
    ),
    ("speed_kilometer_per_hour", UnitId::Speed_KilometerPerHour),
    ("speed_inch_per_second", UnitId::Speed_InchPerSecond),
    ("speed_foot_per_second", UnitId::Speed_FootPerSecond),
    ("speed_mile_per_hour", UnitId::Speed_MilePerHour),
    ("speed_knot", UnitId::Speed_Knot),
    (
        "acceleration_meter_per_second_squared",
        UnitId::Acceleration_MeterPerSecondSquared,
    ),
    (
        "acceleration_millimeter_per_second_squared",
        UnitId::Acceleration_MillimeterPerSecondSquared,
    ),
    (
        "acceleration_centimeter_per_second_squared",
        UnitId::Acceleration_CentimeterPerSecondSquared,
    ),
    (
        "acceleration_inch_per_second_squared",
        UnitId::Acceleration_InchPerSecondSquared,
    ),
    (
        "acceleration_foot_per_second_squared",
        UnitId::Acceleration_FootPerSecondSquared,
    ),
    (
        "acceleration_standard_gravity",
        UnitId::Acceleration_StandardGravity,
    ),
    (
        "angular_velocity_radian_per_second",
        UnitId::AngularVelocity_RadianPerSecond,
    ),
    (
        "angular_velocity_degree_per_second",
        UnitId::AngularVelocity_DegreePerSecond,
    ),
    (
        "angular_velocity_revolution_per_minute",
        UnitId::AngularVelocity_RevolutionPerMinute,
    ),
    (
        "angular_velocity_revolution_per_second",
        UnitId::AngularVelocity_RevolutionPerSecond,
    ),
    ("torque_newton_meter", UnitId::Torque_NewtonMeter),
    ("torque_newton_millimeter", UnitId::Torque_NewtonMillimeter),
    ("torque_newton_centimeter", UnitId::Torque_NewtonCentimeter),
    ("torque_pound_force_inch", UnitId::Torque_PoundForceInch),
    ("torque_pound_force_foot", UnitId::Torque_PoundForceFoot),
    (
        "density_kilogram_per_cubic_meter",
        UnitId::Density_KilogramPerCubicMeter,
    ),
    (
        "density_gram_per_cubic_centimeter",
        UnitId::Density_GramPerCubicCentimeter,
    ),
    ("density_gram_per_liter", UnitId::Density_GramPerLiter),
    (
        "density_kilogram_per_liter",
        UnitId::Density_KilogramPerLiter,
    ),
    (
        "density_pound_per_cubic_foot",
        UnitId::Density_PoundPerCubicFoot,
    ),
    (
        "volume_flow_rate_cubic_meter_per_second",
        UnitId::VolumeFlowRate_CubicMeterPerSecond,
    ),
    (
        "volume_flow_rate_liter_per_minute",
        UnitId::VolumeFlowRate_LiterPerMinute,
    ),
    (
        "displacement_per_revolution_cubic_meter_per_revolution",
        UnitId::DisplacementPerRevolution_CubicMeterPerRevolution,
    ),
    (
        "displacement_per_revolution_cubic_centimeter_per_revolution",
        UnitId::DisplacementPerRevolution_CubicCentimeterPerRevolution,
    ),
    (
        "angular_frequency_radian_per_second",
        UnitId::AngularFrequency_RadianPerSecond,
    ),
    (
        "angular_frequency_picoradian_per_second",
        UnitId::AngularFrequency_PicoradianPerSecond,
    ),
    (
        "angular_frequency_nanoradian_per_second",
        UnitId::AngularFrequency_NanoradianPerSecond,
    ),
    (
        "angular_frequency_microradian_per_second",
        UnitId::AngularFrequency_MicroradianPerSecond,
    ),
    (
        "angular_frequency_milliradian_per_second",
        UnitId::AngularFrequency_MilliradianPerSecond,
    ),
    (
        "angular_frequency_kiloradian_per_second",
        UnitId::AngularFrequency_KiloradianPerSecond,
    ),
    (
        "angular_frequency_megaradian_per_second",
        UnitId::AngularFrequency_MegaradianPerSecond,
    ),
    (
        "angular_frequency_gigaradian_per_second",
        UnitId::AngularFrequency_GigaradianPerSecond,
    ),
    (
        "angular_frequency_teraradian_per_second",
        UnitId::AngularFrequency_TeraradianPerSecond,
    ),
    (
        "moment_of_inertia_kilogram_square_meter",
        UnitId::MomentOfInertia_KilogramSquareMeter,
    ),
    (
        "moment_of_inertia_kilogram_square_centimeter",
        UnitId::MomentOfInertia_KilogramSquareCentimeter,
    ),
    (
        "moment_of_inertia_gram_square_centimeter",
        UnitId::MomentOfInertia_GramSquareCentimeter,
    ),
    (
        "linear_stiffness_newton_per_meter",
        UnitId::LinearStiffness_NewtonPerMeter,
    ),
    (
        "linear_stiffness_newton_per_millimeter",
        UnitId::LinearStiffness_NewtonPerMillimeter,
    ),
    (
        "linear_stiffness_kilonewton_per_meter",
        UnitId::LinearStiffness_KilonewtonPerMeter,
    ),
    (
        "linear_damping_newton_second_per_meter",
        UnitId::LinearDamping_NewtonSecondPerMeter,
    ),
    (
        "linear_damping_newton_second_per_millimeter",
        UnitId::LinearDamping_NewtonSecondPerMillimeter,
    ),
    (
        "linear_damping_kilonewton_second_per_meter",
        UnitId::LinearDamping_KilonewtonSecondPerMeter,
    ),
    (
        "rotational_stiffness_newton_meter_per_radian",
        UnitId::RotationalStiffness_NewtonMeterPerRadian,
    ),
    (
        "rotational_stiffness_newton_millimeter_per_radian",
        UnitId::RotationalStiffness_NewtonMillimeterPerRadian,
    ),
    (
        "rotational_stiffness_kilonewton_meter_per_radian",
        UnitId::RotationalStiffness_KilonewtonMeterPerRadian,
    ),
    (
        "rotational_damping_newton_meter_second_per_radian",
        UnitId::RotationalDamping_NewtonMeterSecondPerRadian,
    ),
    (
        "rotational_damping_newton_millimeter_second_per_radian",
        UnitId::RotationalDamping_NewtonMillimeterSecondPerRadian,
    ),
    (
        "rotational_damping_kilonewton_meter_second_per_radian",
        UnitId::RotationalDamping_KilonewtonMeterSecondPerRadian,
    ),
    (
        "dynamic_viscosity_pascal_second",
        UnitId::DynamicViscosity_PascalSecond,
    ),
    (
        "dynamic_viscosity_millipascal_second",
        UnitId::DynamicViscosity_MillipascalSecond,
    ),
    ("dynamic_viscosity_poise", UnitId::DynamicViscosity_Poise),
    (
        "dynamic_viscosity_centipoise",
        UnitId::DynamicViscosity_Centipoise,
    ),
    (
        "kinematic_viscosity_square_meter_per_second",
        UnitId::KinematicViscosity_SquareMeterPerSecond,
    ),
    (
        "kinematic_viscosity_square_millimeter_per_second",
        UnitId::KinematicViscosity_SquareMillimeterPerSecond,
    ),
    (
        "kinematic_viscosity_stokes",
        UnitId::KinematicViscosity_Stokes,
    ),
    (
        "kinematic_viscosity_centistokes",
        UnitId::KinematicViscosity_Centistokes,
    ),
    (
        "mass_flow_rate_kilogram_per_second",
        UnitId::MassFlowRate_KilogramPerSecond,
    ),
    (
        "mass_flow_rate_gram_per_second",
        UnitId::MassFlowRate_GramPerSecond,
    ),
    (
        "mass_flow_rate_kilogram_per_minute",
        UnitId::MassFlowRate_KilogramPerMinute,
    ),
    (
        "mass_flow_rate_kilogram_per_hour",
        UnitId::MassFlowRate_KilogramPerHour,
    ),
    (
        "momentum_kilogram_meter_per_second",
        UnitId::Momentum_KilogramMeterPerSecond,
    ),
    (
        "momentum_gram_centimeter_per_second",
        UnitId::Momentum_GramCentimeterPerSecond,
    ),
    ("momentum_newton_second", UnitId::Momentum_NewtonSecond),
    (
        "hydraulic_leakage_coefficient_cubic_meter_per_second_per_pascal",
        UnitId::HydraulicLeakageCoefficient_CubicMeterPerSecondPerPascal,
    ),
    (
        "hydraulic_leakage_coefficient_liter_per_minute_per_bar",
        UnitId::HydraulicLeakageCoefficient_LiterPerMinutePerBar,
    ),
    (
        "specific_heat_capacity_joule_per_kilogram_kelvin",
        UnitId::SpecificHeatCapacity_JoulePerKilogramKelvin,
    ),
    (
        "specific_heat_capacity_kilojoule_per_kilogram_kelvin",
        UnitId::SpecificHeatCapacity_KilojoulePerKilogramKelvin,
    ),
    (
        "specific_gas_constant_joule_per_kilogram_kelvin",
        UnitId::SpecificGasConstant_JoulePerKilogramKelvin,
    ),
    (
        "specific_gas_constant_kilojoule_per_kilogram_kelvin",
        UnitId::SpecificGasConstant_KilojoulePerKilogramKelvin,
    ),
    (
        "thermal_conductance_watt_per_kelvin",
        UnitId::ThermalConductance_WattPerKelvin,
    ),
    (
        "thermal_conductance_milliwatt_per_kelvin",
        UnitId::ThermalConductance_MilliwattPerKelvin,
    ),
    (
        "thermal_conductance_kilowatt_per_kelvin",
        UnitId::ThermalConductance_KilowattPerKelvin,
    ),
    (
        "hydraulic_resistance_pascal_second_per_cubic_meter",
        UnitId::HydraulicResistance_PascalSecondPerCubicMeter,
    ),
    (
        "hydraulic_resistance_bar_minute_per_liter",
        UnitId::HydraulicResistance_BarMinutePerLiter,
    ),
    (
        "pneumatic_characteristic_impedance_pascal_second_per_joule",
        UnitId::PneumaticCharacteristicImpedance_PascalSecondPerJoule,
    ),
    (
        "pneumatic_characteristic_impedance_second_per_cubic_meter",
        UnitId::PneumaticCharacteristicImpedance_SecondPerCubicMeter,
    ),
    (
        "turbulent_flow_coefficient_cubic_meter_per_second_per_square_root_pascal",
        UnitId::TurbulentFlowCoefficient_CubicMeterPerSecondPerSquareRootPascal,
    ),
    (
        "turbulent_flow_coefficient_liter_per_minute_per_square_root_bar",
        UnitId::TurbulentFlowCoefficient_LiterPerMinutePerSquareRootBar,
    ),
    (
        "motor_back_emf_constant_volt_second_per_radian",
        UnitId::MotorBackEmfConstant_VoltSecondPerRadian,
    ),
    (
        "motor_back_emf_constant_millivolt_second_per_radian",
        UnitId::MotorBackEmfConstant_MillivoltSecondPerRadian,
    ),
    (
        "motor_back_emf_constant_volt_per_revolution_per_minute",
        UnitId::MotorBackEmfConstant_VoltPerRevolutionPerMinute,
    ),
    (
        "screw_pitch_meter_per_radian",
        UnitId::ScrewPitch_MeterPerRadian,
    ),
    (
        "screw_pitch_millimeter_per_radian",
        UnitId::ScrewPitch_MillimeterPerRadian,
    ),
    (
        "screw_pitch_millimeter_per_revolution",
        UnitId::ScrewPitch_MillimeterPerRevolution,
    ),
    (
        "thrust_specific_fuel_consumption_kilogram_per_newton_second",
        UnitId::ThrustSpecificFuelConsumption_KilogramPerNewtonSecond,
    ),
    (
        "thrust_specific_fuel_consumption_kilogram_per_kilonewton_hour",
        UnitId::ThrustSpecificFuelConsumption_KilogramPerKilonewtonHour,
    ),
    (
        "volume_flow_acceleration_cubic_meter_per_second_squared",
        UnitId::VolumeFlowAcceleration_CubicMeterPerSecondSquared,
    ),
    (
        "volume_flow_acceleration_liter_per_minute_per_second",
        UnitId::VolumeFlowAcceleration_LiterPerMinutePerSecond,
    ),
];

#[cfg(test)]
mod tests {
    use super::{UNIT_IDS_V1, UnitIdEnumReaderV1};
    use hrafnix_units::UnitId;
    use std::collections::HashSet;

    #[test]
    fn every_snapshot_id_deserializes_and_maps_to_its_unit() {
        assert_eq!(UNIT_IDS_V1.len(), 314);
        let mut ids = HashSet::new();
        let mut units = HashSet::new();

        for &(id, expected) in UNIT_IDS_V1 {
            assert!(ids.insert(id), "duplicate version-1 identifier {id}");
            assert!(
                units.insert(expected),
                "duplicate unit mapping {expected:?}"
            );
            let json = serde_json::to_string(id).unwrap();
            let unit: UnitIdEnumReaderV1 = serde_json::from_str(&json).unwrap();
            assert_eq!(UnitId::from(unit), expected, "{id}");
            let prefixed = serde_json::to_string(&format!("u_{id}")).unwrap();
            assert!(serde_json::from_str::<UnitIdEnumReaderV1>(&prefixed).is_err());
        }
    }

    #[test]
    fn unknown_unit_ids_are_rejected() {
        for id in [
            "",
            "unknown",
            "length_femtometer",
            "Length_Meter",
            "LengthMeter",
            "ULengthMeter",
            "U_Length_Meter",
            "U_LENGTH_METER",
            "length_meter ",
            "100",
        ] {
            let json = serde_json::to_string(id).unwrap();
            let error = serde_json::from_str::<UnitIdEnumReaderV1>(&json).unwrap_err();
            assert!(error.to_string().contains("unknown variant"), "{error}");
        }
    }

    #[test]
    fn unit_ids_must_be_plain_strings() {
        for json in [
            "null",
            "true",
            "0",
            "100",
            "100.0",
            "[]",
            r#"["length_meter"]"#,
            "{}",
            r#"{"length_meter": null}"#,
            r#"{"Length_Meter": null}"#,
            r#"{"LengthMeter": null}"#,
            r#"{"ULengthMeter": null}"#,
        ] {
            let error = serde_json::from_str::<UnitIdEnumReaderV1>(json).unwrap_err();
            assert!(error.to_string().contains("expected a string"), "{error}");
        }
    }
}

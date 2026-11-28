//! Frozen version-1 unit identifiers, independent of runtime keys and labels.

use hrafnix_units::UnitId;
use serde::Serialize;

/// A unit represented by its plain-string version-1 identifier.
#[derive(Debug, Serialize)]
pub(crate) struct UnitIdDefinitionWriterV1(&'static str);

impl From<UnitId> for UnitIdDefinitionWriterV1 {
    fn from(unit: UnitId) -> Self {
        Self(match unit {
            UnitId::None => "none",
            UnitId::Length_Meter => "length_meter",
            UnitId::Length_Picometer => "length_picometer",
            UnitId::Length_Nanometer => "length_nanometer",
            UnitId::Length_Micrometer => "length_micrometer",
            UnitId::Length_Millimeter => "length_millimeter",
            UnitId::Length_Centimeter => "length_centimeter",
            UnitId::Length_Kilometer => "length_kilometer",
            UnitId::Length_Foot => "length_foot",
            UnitId::Length_Inch => "length_inch",
            UnitId::Length_Yard => "length_yard",
            UnitId::Length_Mile => "length_mile",
            UnitId::Mass_Kilogram => "mass_kilogram",
            UnitId::Mass_Gram => "mass_gram",
            UnitId::Mass_Picogram => "mass_picogram",
            UnitId::Mass_Nanogram => "mass_nanogram",
            UnitId::Mass_Microgram => "mass_microgram",
            UnitId::Mass_Milligram => "mass_milligram",
            UnitId::Mass_Megagram => "mass_megagram",
            UnitId::Mass_Tonne => "mass_tonne",
            UnitId::Mass_Pound => "mass_pound",
            UnitId::Mass_Ounce => "mass_ounce",
            UnitId::Mass_Stone => "mass_stone",
            UnitId::Time_Second => "time_second",
            UnitId::Time_Picosecond => "time_picosecond",
            UnitId::Time_Nanosecond => "time_nanosecond",
            UnitId::Time_Microsecond => "time_microsecond",
            UnitId::Time_Millisecond => "time_millisecond",
            UnitId::Time_Kilosecond => "time_kilosecond",
            UnitId::Time_Minute => "time_minute",
            UnitId::Time_Hour => "time_hour",
            UnitId::Time_Day => "time_day",
            UnitId::Time_Week => "time_week",
            UnitId::Time_Year => "time_year",
            UnitId::Current_Ampere => "current_ampere",
            UnitId::Current_Nanoampere => "current_nanoampere",
            UnitId::Current_Microampere => "current_microampere",
            UnitId::Current_Milliampere => "current_milliampere",
            UnitId::Current_Kiloampere => "current_kiloampere",
            UnitId::Current_Megaampere => "current_megaampere",
            UnitId::Temperature_Kelvin => "temperature_kelvin",
            UnitId::Temperature_Nanokelvin => "temperature_nanokelvin",
            UnitId::Temperature_Microkelvin => "temperature_microkelvin",
            UnitId::Temperature_Millikelvin => "temperature_millikelvin",
            UnitId::Temperature_Kilokelvin => "temperature_kilokelvin",
            UnitId::Temperature_Celsius => "temperature_celsius",
            UnitId::Temperature_Fahrenheit => "temperature_fahrenheit",
            UnitId::Amount_Mole => "amount_mole",
            UnitId::Amount_Picomole => "amount_picomole",
            UnitId::Amount_Nanomole => "amount_nanomole",
            UnitId::Amount_Micromole => "amount_micromole",
            UnitId::Amount_Millimole => "amount_millimole",
            UnitId::Amount_Kilomole => "amount_kilomole",
            UnitId::Frequency_Hertz => "frequency_hertz",
            UnitId::Frequency_Picohertz => "frequency_picohertz",
            UnitId::Frequency_Nanohertz => "frequency_nanohertz",
            UnitId::Frequency_Microhertz => "frequency_microhertz",
            UnitId::Frequency_Millihertz => "frequency_millihertz",
            UnitId::Frequency_Kilohertz => "frequency_kilohertz",
            UnitId::Frequency_Megahertz => "frequency_megahertz",
            UnitId::Frequency_Gigahertz => "frequency_gigahertz",
            UnitId::Frequency_Terahertz => "frequency_terahertz",
            UnitId::Angle_Radian => "angle_radian",
            UnitId::Angle_Picoradian => "angle_picoradian",
            UnitId::Angle_Nanoradian => "angle_nanoradian",
            UnitId::Angle_Microradian => "angle_microradian",
            UnitId::Angle_Milliradian => "angle_milliradian",
            UnitId::Angle_Kiloradian => "angle_kiloradian",
            UnitId::Angle_Megaradian => "angle_megaradian",
            UnitId::Angle_Gigaradian => "angle_gigaradian",
            UnitId::Angle_Teraradian => "angle_teraradian",
            UnitId::Angle_Degree => "angle_degree",
            UnitId::Angle_Turn => "angle_turn",
            UnitId::Angle_Arcminute => "angle_arcminute",
            UnitId::Angle_Arcsecond => "angle_arcsecond",
            UnitId::SolidAngle_Steradian => "solid_angle_steradian",
            UnitId::SolidAngle_Picosteradian => "solid_angle_picosteradian",
            UnitId::SolidAngle_Nanosteradian => "solid_angle_nanosteradian",
            UnitId::SolidAngle_Microsteradian => "solid_angle_microsteradian",
            UnitId::SolidAngle_Millisteradian => "solid_angle_millisteradian",
            UnitId::Force_Newton => "force_newton",
            UnitId::Force_Piconewton => "force_piconewton",
            UnitId::Force_Nanonewton => "force_nanonewton",
            UnitId::Force_Micronewton => "force_micronewton",
            UnitId::Force_Millinewton => "force_millinewton",
            UnitId::Force_Kilonewton => "force_kilonewton",
            UnitId::Force_Meganewton => "force_meganewton",
            UnitId::Force_Giganewton => "force_giganewton",
            UnitId::Force_Teranewton => "force_teranewton",
            UnitId::Force_KilogramForce => "force_kilogram_force",
            UnitId::Force_PoundForce => "force_pound_force",
            UnitId::Pressure_Pascal => "pressure_pascal",
            UnitId::Pressure_Picopascal => "pressure_picopascal",
            UnitId::Pressure_Nanopascal => "pressure_nanopascal",
            UnitId::Pressure_Micropascal => "pressure_micropascal",
            UnitId::Pressure_Millipascal => "pressure_millipascal",
            UnitId::Pressure_Kilopascal => "pressure_kilopascal",
            UnitId::Pressure_Megapascal => "pressure_megapascal",
            UnitId::Pressure_Gigapascal => "pressure_gigapascal",
            UnitId::Pressure_Terapascal => "pressure_terapascal",
            UnitId::Pressure_Bar => "pressure_bar",
            UnitId::Pressure_Millibar => "pressure_millibar",
            UnitId::Pressure_StandardAtmosphere => "pressure_standard_atmosphere",
            UnitId::Pressure_PoundPerSquareInch => "pressure_pound_per_square_inch",
            UnitId::Pressure_MillimeterOfMercury => "pressure_millimeter_of_mercury",
            UnitId::Energy_Joule => "energy_joule",
            UnitId::Energy_Picojoule => "energy_picojoule",
            UnitId::Energy_Nanojoule => "energy_nanojoule",
            UnitId::Energy_Microjoule => "energy_microjoule",
            UnitId::Energy_Millijoule => "energy_millijoule",
            UnitId::Energy_Kilojoule => "energy_kilojoule",
            UnitId::Energy_Megajoule => "energy_megajoule",
            UnitId::Energy_Gigajoule => "energy_gigajoule",
            UnitId::Energy_Terajoule => "energy_terajoule",
            UnitId::Energy_WattHour => "energy_watt_hour",
            UnitId::Energy_KilowattHour => "energy_kilowatt_hour",
            UnitId::Energy_Electronvolt => "energy_electronvolt",
            UnitId::Power_Watt => "power_watt",
            UnitId::Power_Picowatt => "power_picowatt",
            UnitId::Power_Nanowatt => "power_nanowatt",
            UnitId::Power_Microwatt => "power_microwatt",
            UnitId::Power_Milliwatt => "power_milliwatt",
            UnitId::Power_Kilowatt => "power_kilowatt",
            UnitId::Power_Megawatt => "power_megawatt",
            UnitId::Power_Gigawatt => "power_gigawatt",
            UnitId::Power_Terawatt => "power_terawatt",
            UnitId::Power_MechanicalHorsepower => "power_mechanical_horsepower",
            UnitId::Power_MetricHorsepower => "power_metric_horsepower",
            UnitId::ElectricCharge_Coulomb => "electric_charge_coulomb",
            UnitId::ElectricCharge_Picocoulomb => "electric_charge_picocoulomb",
            UnitId::ElectricCharge_Nanocoulomb => "electric_charge_nanocoulomb",
            UnitId::ElectricCharge_Microcoulomb => "electric_charge_microcoulomb",
            UnitId::ElectricCharge_Millicoulomb => "electric_charge_millicoulomb",
            UnitId::ElectricCharge_Kilocoulomb => "electric_charge_kilocoulomb",
            UnitId::ElectricCharge_Megacoulomb => "electric_charge_megacoulomb",
            UnitId::ElectricCharge_Gigacoulomb => "electric_charge_gigacoulomb",
            UnitId::ElectricCharge_Teracoulomb => "electric_charge_teracoulomb",
            UnitId::ElectricCharge_AmpereHour => "electric_charge_ampere_hour",
            UnitId::ElectricCharge_MilliampereHour => "electric_charge_milliampere_hour",
            UnitId::Voltage_Volt => "voltage_volt",
            UnitId::Voltage_Picovolt => "voltage_picovolt",
            UnitId::Voltage_Nanovolt => "voltage_nanovolt",
            UnitId::Voltage_Microvolt => "voltage_microvolt",
            UnitId::Voltage_Millivolt => "voltage_millivolt",
            UnitId::Voltage_Kilovolt => "voltage_kilovolt",
            UnitId::Voltage_Megavolt => "voltage_megavolt",
            UnitId::Voltage_Gigavolt => "voltage_gigavolt",
            UnitId::Voltage_Teravolt => "voltage_teravolt",
            UnitId::Resistance_Ohm => "resistance_ohm",
            UnitId::Resistance_Picoohm => "resistance_picoohm",
            UnitId::Resistance_Nanoohm => "resistance_nanoohm",
            UnitId::Resistance_Microohm => "resistance_microohm",
            UnitId::Resistance_Milliohm => "resistance_milliohm",
            UnitId::Resistance_Kiloohm => "resistance_kiloohm",
            UnitId::Resistance_Megaohm => "resistance_megaohm",
            UnitId::Resistance_Gigaohm => "resistance_gigaohm",
            UnitId::Resistance_Teraohm => "resistance_teraohm",
            UnitId::Conductance_Siemens => "conductance_siemens",
            UnitId::Conductance_Picosiemens => "conductance_picosiemens",
            UnitId::Conductance_Nanosiemens => "conductance_nanosiemens",
            UnitId::Conductance_Microsiemens => "conductance_microsiemens",
            UnitId::Conductance_Millisiemens => "conductance_millisiemens",
            UnitId::Conductance_Kilosiemens => "conductance_kilosiemens",
            UnitId::Conductance_Megasiemens => "conductance_megasiemens",
            UnitId::Conductance_Gigasiemens => "conductance_gigasiemens",
            UnitId::Conductance_Terasiemens => "conductance_terasiemens",
            UnitId::Capacitance_Farad => "capacitance_farad",
            UnitId::Capacitance_Picofarad => "capacitance_picofarad",
            UnitId::Capacitance_Nanofarad => "capacitance_nanofarad",
            UnitId::Capacitance_Microfarad => "capacitance_microfarad",
            UnitId::Capacitance_Millifarad => "capacitance_millifarad",
            UnitId::Capacitance_Kilofarad => "capacitance_kilofarad",
            UnitId::Inductance_Henry => "inductance_henry",
            UnitId::Inductance_Picohenry => "inductance_picohenry",
            UnitId::Inductance_Nanohenry => "inductance_nanohenry",
            UnitId::Inductance_Microhenry => "inductance_microhenry",
            UnitId::Inductance_Millihenry => "inductance_millihenry",
            UnitId::MagneticFluxDensity_Tesla => "magnetic_flux_density_tesla",
            UnitId::MagneticFluxDensity_Picotesla => "magnetic_flux_density_picotesla",
            UnitId::MagneticFluxDensity_Nanotesla => "magnetic_flux_density_nanotesla",
            UnitId::MagneticFluxDensity_Microtesla => "magnetic_flux_density_microtesla",
            UnitId::MagneticFluxDensity_Millitesla => "magnetic_flux_density_millitesla",
            UnitId::MagneticFlux_Weber => "magnetic_flux_weber",
            UnitId::MagneticFlux_Picoweber => "magnetic_flux_picoweber",
            UnitId::MagneticFlux_Nanoweber => "magnetic_flux_nanoweber",
            UnitId::MagneticFlux_Microweber => "magnetic_flux_microweber",
            UnitId::MagneticFlux_Milliweber => "magnetic_flux_milliweber",
            UnitId::MagneticFlux_Kiloweber => "magnetic_flux_kiloweber",
            UnitId::MagneticFlux_Megaweber => "magnetic_flux_megaweber",
            UnitId::MagneticFlux_Gigaweber => "magnetic_flux_gigaweber",
            UnitId::MagneticFlux_Teraweber => "magnetic_flux_teraweber",
            UnitId::Area_SquareMeter => "area_square_meter",
            UnitId::Area_SquareMicrometer => "area_square_micrometer",
            UnitId::Area_SquareMillimeter => "area_square_millimeter",
            UnitId::Area_SquareCentimeter => "area_square_centimeter",
            UnitId::Area_SquareKilometer => "area_square_kilometer",
            UnitId::Area_SquareFoot => "area_square_foot",
            UnitId::Area_SquareInch => "area_square_inch",
            UnitId::Area_Acre => "area_acre",
            UnitId::Area_Hectare => "area_hectare",
            UnitId::Area_SquareMile => "area_square_mile",
            UnitId::Volume_CubicMeter => "volume_cubic_meter",
            UnitId::Volume_CubicCentimeter => "volume_cubic_centimeter",
            UnitId::Volume_CubicMillimeter => "volume_cubic_millimeter",
            UnitId::Volume_Liter => "volume_liter",
            UnitId::Volume_Picoliter => "volume_picoliter",
            UnitId::Volume_Nanoliter => "volume_nanoliter",
            UnitId::Volume_Microliter => "volume_microliter",
            UnitId::Volume_Milliliter => "volume_milliliter",
            UnitId::Volume_Kiloliter => "volume_kiloliter",
            UnitId::Volume_Megaliter => "volume_megaliter",
            UnitId::Volume_Gallon => "volume_gallon",
            UnitId::Volume_ImperialGallon => "volume_imperial_gallon",
            UnitId::Volume_FluidOunce => "volume_fluid_ounce",
            UnitId::Volume_ImperialFluidOunce => "volume_imperial_fluid_ounce",
            UnitId::Volume_Cup => "volume_cup",
            UnitId::Volume_Pint => "volume_pint",
            UnitId::Volume_Quart => "volume_quart",
            UnitId::Speed_MeterPerSecond => "speed_meter_per_second",
            UnitId::Speed_MillimeterPerSecond => "speed_millimeter_per_second",
            UnitId::Speed_CentimeterPerSecond => "speed_centimeter_per_second",
            UnitId::Speed_KilometerPerHour => "speed_kilometer_per_hour",
            UnitId::Speed_InchPerSecond => "speed_inch_per_second",
            UnitId::Speed_FootPerSecond => "speed_foot_per_second",
            UnitId::Speed_MilePerHour => "speed_mile_per_hour",
            UnitId::Speed_Knot => "speed_knot",
            UnitId::Acceleration_MeterPerSecondSquared => "acceleration_meter_per_second_squared",
            UnitId::Acceleration_MillimeterPerSecondSquared => {
                "acceleration_millimeter_per_second_squared"
            }
            UnitId::Acceleration_CentimeterPerSecondSquared => {
                "acceleration_centimeter_per_second_squared"
            }
            UnitId::Acceleration_InchPerSecondSquared => "acceleration_inch_per_second_squared",
            UnitId::Acceleration_FootPerSecondSquared => "acceleration_foot_per_second_squared",
            UnitId::Acceleration_StandardGravity => "acceleration_standard_gravity",
            UnitId::AngularVelocity_RadianPerSecond => "angular_velocity_radian_per_second",
            UnitId::AngularVelocity_DegreePerSecond => "angular_velocity_degree_per_second",
            UnitId::AngularVelocity_RevolutionPerMinute => "angular_velocity_revolution_per_minute",
            UnitId::AngularVelocity_RevolutionPerSecond => "angular_velocity_revolution_per_second",
            UnitId::Torque_NewtonMeter => "torque_newton_meter",
            UnitId::Torque_NewtonMillimeter => "torque_newton_millimeter",
            UnitId::Torque_NewtonCentimeter => "torque_newton_centimeter",
            UnitId::Torque_PoundForceInch => "torque_pound_force_inch",
            UnitId::Torque_PoundForceFoot => "torque_pound_force_foot",
            UnitId::Density_KilogramPerCubicMeter => "density_kilogram_per_cubic_meter",
            UnitId::Density_GramPerCubicCentimeter => "density_gram_per_cubic_centimeter",
            UnitId::Density_GramPerLiter => "density_gram_per_liter",
            UnitId::Density_KilogramPerLiter => "density_kilogram_per_liter",
            UnitId::Density_PoundPerCubicFoot => "density_pound_per_cubic_foot",
            UnitId::VolumeFlowRate_CubicMeterPerSecond => "volume_flow_rate_cubic_meter_per_second",
            UnitId::VolumeFlowRate_LiterPerMinute => "volume_flow_rate_liter_per_minute",
            UnitId::DisplacementPerRevolution_CubicMeterPerRevolution => {
                "displacement_per_revolution_cubic_meter_per_revolution"
            }
            UnitId::DisplacementPerRevolution_CubicCentimeterPerRevolution => {
                "displacement_per_revolution_cubic_centimeter_per_revolution"
            }
            UnitId::AngularFrequency_RadianPerSecond => "angular_frequency_radian_per_second",
            UnitId::AngularFrequency_PicoradianPerSecond => {
                "angular_frequency_picoradian_per_second"
            }
            UnitId::AngularFrequency_NanoradianPerSecond => {
                "angular_frequency_nanoradian_per_second"
            }
            UnitId::AngularFrequency_MicroradianPerSecond => {
                "angular_frequency_microradian_per_second"
            }
            UnitId::AngularFrequency_MilliradianPerSecond => {
                "angular_frequency_milliradian_per_second"
            }
            UnitId::AngularFrequency_KiloradianPerSecond => {
                "angular_frequency_kiloradian_per_second"
            }
            UnitId::AngularFrequency_MegaradianPerSecond => {
                "angular_frequency_megaradian_per_second"
            }
            UnitId::AngularFrequency_GigaradianPerSecond => {
                "angular_frequency_gigaradian_per_second"
            }
            UnitId::AngularFrequency_TeraradianPerSecond => {
                "angular_frequency_teraradian_per_second"
            }
            UnitId::MomentOfInertia_KilogramSquareMeter => {
                "moment_of_inertia_kilogram_square_meter"
            }
            UnitId::MomentOfInertia_KilogramSquareCentimeter => {
                "moment_of_inertia_kilogram_square_centimeter"
            }
            UnitId::MomentOfInertia_GramSquareCentimeter => {
                "moment_of_inertia_gram_square_centimeter"
            }
            UnitId::LinearStiffness_NewtonPerMeter => "linear_stiffness_newton_per_meter",
            UnitId::LinearStiffness_NewtonPerMillimeter => "linear_stiffness_newton_per_millimeter",
            UnitId::LinearStiffness_KilonewtonPerMeter => "linear_stiffness_kilonewton_per_meter",
            UnitId::LinearDamping_NewtonSecondPerMeter => "linear_damping_newton_second_per_meter",
            UnitId::LinearDamping_NewtonSecondPerMillimeter => {
                "linear_damping_newton_second_per_millimeter"
            }
            UnitId::LinearDamping_KilonewtonSecondPerMeter => {
                "linear_damping_kilonewton_second_per_meter"
            }
            UnitId::RotationalStiffness_NewtonMeterPerRadian => {
                "rotational_stiffness_newton_meter_per_radian"
            }
            UnitId::RotationalStiffness_NewtonMillimeterPerRadian => {
                "rotational_stiffness_newton_millimeter_per_radian"
            }
            UnitId::RotationalStiffness_KilonewtonMeterPerRadian => {
                "rotational_stiffness_kilonewton_meter_per_radian"
            }
            UnitId::RotationalDamping_NewtonMeterSecondPerRadian => {
                "rotational_damping_newton_meter_second_per_radian"
            }
            UnitId::RotationalDamping_NewtonMillimeterSecondPerRadian => {
                "rotational_damping_newton_millimeter_second_per_radian"
            }
            UnitId::RotationalDamping_KilonewtonMeterSecondPerRadian => {
                "rotational_damping_kilonewton_meter_second_per_radian"
            }
            UnitId::DynamicViscosity_PascalSecond => "dynamic_viscosity_pascal_second",
            UnitId::DynamicViscosity_MillipascalSecond => "dynamic_viscosity_millipascal_second",
            UnitId::DynamicViscosity_Poise => "dynamic_viscosity_poise",
            UnitId::DynamicViscosity_Centipoise => "dynamic_viscosity_centipoise",
            UnitId::KinematicViscosity_SquareMeterPerSecond => {
                "kinematic_viscosity_square_meter_per_second"
            }
            UnitId::KinematicViscosity_SquareMillimeterPerSecond => {
                "kinematic_viscosity_square_millimeter_per_second"
            }
            UnitId::KinematicViscosity_Stokes => "kinematic_viscosity_stokes",
            UnitId::KinematicViscosity_Centistokes => "kinematic_viscosity_centistokes",
            UnitId::MassFlowRate_KilogramPerSecond => "mass_flow_rate_kilogram_per_second",
            UnitId::MassFlowRate_GramPerSecond => "mass_flow_rate_gram_per_second",
            UnitId::MassFlowRate_KilogramPerMinute => "mass_flow_rate_kilogram_per_minute",
            UnitId::MassFlowRate_KilogramPerHour => "mass_flow_rate_kilogram_per_hour",
            UnitId::Momentum_KilogramMeterPerSecond => "momentum_kilogram_meter_per_second",
            UnitId::Momentum_GramCentimeterPerSecond => "momentum_gram_centimeter_per_second",
            UnitId::Momentum_NewtonSecond => "momentum_newton_second",
            UnitId::HydraulicLeakageCoefficient_CubicMeterPerSecondPerPascal => {
                "hydraulic_leakage_coefficient_cubic_meter_per_second_per_pascal"
            }
            UnitId::HydraulicLeakageCoefficient_LiterPerMinutePerBar => {
                "hydraulic_leakage_coefficient_liter_per_minute_per_bar"
            }
            UnitId::SpecificHeatCapacity_JoulePerKilogramKelvin => {
                "specific_heat_capacity_joule_per_kilogram_kelvin"
            }
            UnitId::SpecificHeatCapacity_KilojoulePerKilogramKelvin => {
                "specific_heat_capacity_kilojoule_per_kilogram_kelvin"
            }
            UnitId::SpecificGasConstant_JoulePerKilogramKelvin => {
                "specific_gas_constant_joule_per_kilogram_kelvin"
            }
            UnitId::SpecificGasConstant_KilojoulePerKilogramKelvin => {
                "specific_gas_constant_kilojoule_per_kilogram_kelvin"
            }
            UnitId::ThermalConductance_WattPerKelvin => "thermal_conductance_watt_per_kelvin",
            UnitId::ThermalConductance_MilliwattPerKelvin => {
                "thermal_conductance_milliwatt_per_kelvin"
            }
            UnitId::ThermalConductance_KilowattPerKelvin => {
                "thermal_conductance_kilowatt_per_kelvin"
            }
            UnitId::HydraulicResistance_PascalSecondPerCubicMeter => {
                "hydraulic_resistance_pascal_second_per_cubic_meter"
            }
            UnitId::HydraulicResistance_BarMinutePerLiter => {
                "hydraulic_resistance_bar_minute_per_liter"
            }
            UnitId::PneumaticCharacteristicImpedance_PascalSecondPerJoule => {
                "pneumatic_characteristic_impedance_pascal_second_per_joule"
            }
            UnitId::PneumaticCharacteristicImpedance_SecondPerCubicMeter => {
                "pneumatic_characteristic_impedance_second_per_cubic_meter"
            }
            UnitId::TurbulentFlowCoefficient_CubicMeterPerSecondPerSquareRootPascal => {
                "turbulent_flow_coefficient_cubic_meter_per_second_per_square_root_pascal"
            }
            UnitId::TurbulentFlowCoefficient_LiterPerMinutePerSquareRootBar => {
                "turbulent_flow_coefficient_liter_per_minute_per_square_root_bar"
            }
            UnitId::MotorBackEmfConstant_VoltSecondPerRadian => {
                "motor_back_emf_constant_volt_second_per_radian"
            }
            UnitId::MotorBackEmfConstant_MillivoltSecondPerRadian => {
                "motor_back_emf_constant_millivolt_second_per_radian"
            }
            UnitId::MotorBackEmfConstant_VoltPerRevolutionPerMinute => {
                "motor_back_emf_constant_volt_per_revolution_per_minute"
            }
            UnitId::ScrewPitch_MeterPerRadian => "screw_pitch_meter_per_radian",
            UnitId::ScrewPitch_MillimeterPerRadian => "screw_pitch_millimeter_per_radian",
            UnitId::ScrewPitch_MillimeterPerRevolution => "screw_pitch_millimeter_per_revolution",
            UnitId::ThrustSpecificFuelConsumption_KilogramPerNewtonSecond => {
                "thrust_specific_fuel_consumption_kilogram_per_newton_second"
            }
            UnitId::ThrustSpecificFuelConsumption_KilogramPerKilonewtonHour => {
                "thrust_specific_fuel_consumption_kilogram_per_kilonewton_hour"
            }
            UnitId::VolumeFlowAcceleration_CubicMeterPerSecondSquared => {
                "volume_flow_acceleration_cubic_meter_per_second_squared"
            }
            UnitId::VolumeFlowAcceleration_LiterPerMinutePerSecond => {
                "volume_flow_acceleration_liter_per_minute_per_second"
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn writes_frozen_unit_identifiers_as_plain_strings() {
        for (unit, expected) in [
            (UnitId::None, "none"),
            (UnitId::Length_Meter, "length_meter"),
            (UnitId::Length_Centimeter, "length_centimeter"),
            (UnitId::Time_Hour, "time_hour"),
            (UnitId::Temperature_Celsius, "temperature_celsius"),
            (
                UnitId::Pressure_PoundPerSquareInch,
                "pressure_pound_per_square_inch",
            ),
            (
                UnitId::VolumeFlowAcceleration_LiterPerMinutePerSecond,
                "volume_flow_acceleration_liter_per_minute_per_second",
            ),
        ] {
            assert_eq!(
                serde_json::to_value(UnitIdDefinitionWriterV1::from(unit)).unwrap(),
                json!(expected),
                "{unit:?}"
            );
        }
    }
}

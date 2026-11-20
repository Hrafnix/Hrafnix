use crate::common::serialize::serialize_shareable_string;
use hrafnix_datastore::definition::UnitDefinition;
use hrafnix_shareable_string::ShareableString;
use hrafnix_units::UnitFamilyId;
use serde::Serialize;

/// Version-1 unit selection expression and family.
#[derive(Debug, Serialize)]
pub(crate) struct UnitDefinitionWriterV1 {
    /// Item description.
    #[serde(serialize_with = "serialize_shareable_string")]
    description: ShareableString,
    /// Default expression.
    #[serde(serialize_with = "serialize_shareable_string")]
    default: ShareableString,
    /// Unit family as a plain snake-case string.
    #[serde(with = "UnitFamilyDefinitionWriterV1")]
    unit_family: UnitFamilyId,
}

impl From<&UnitDefinition> for UnitDefinitionWriterV1 {
    fn from(unit: &UnitDefinition) -> Self {
        Self {
            description: unit.description(),
            default: unit.default_value(),
            unit_family: unit.unit_family(),
        }
    }
}

/// Frozen version-1 family identifiers, independent of runtime display names.
#[derive(Debug, Serialize)]
#[serde(remote = "UnitFamilyId", rename_all = "snake_case")]
enum UnitFamilyDefinitionWriterV1 {
    /// No units.
    None,
    /// Length.
    Length,
    /// Mass.
    Mass,
    /// Time.
    Time,
    /// Electric current.
    Current,
    /// Temperature.
    Temperature,
    /// Amount of substance.
    Amount,
    /// Frequency.
    Frequency,
    /// Angle.
    Angle,
    /// Solid angle.
    SolidAngle,
    /// Force.
    Force,
    /// Pressure.
    Pressure,
    /// Energy.
    Energy,
    /// Power.
    Power,
    /// Electric charge.
    ElectricCharge,
    /// Voltage.
    Voltage,
    /// Resistance.
    Resistance,
    /// Conductance.
    Conductance,
    /// Capacitance.
    Capacitance,
    /// Inductance.
    Inductance,
    /// Magnetic flux density.
    MagneticFluxDensity,
    /// Magnetic flux.
    MagneticFlux,
    /// Area.
    Area,
    /// Volume.
    Volume,
    /// Speed.
    Speed,
    /// Acceleration.
    Acceleration,
    /// Angular velocity.
    AngularVelocity,
    /// Torque.
    Torque,
    /// Density.
    Density,
    /// Volume flow rate.
    VolumeFlowRate,
    /// Displacement per revolution.
    DisplacementPerRevolution,
    /// Angular frequency.
    AngularFrequency,
    /// Moment of inertia.
    MomentOfInertia,
    /// Linear stiffness.
    LinearStiffness,
    /// Linear damping.
    LinearDamping,
    /// Rotational stiffness.
    RotationalStiffness,
    /// Rotational damping.
    RotationalDamping,
    /// Dynamic viscosity.
    DynamicViscosity,
    /// Kinematic viscosity.
    KinematicViscosity,
    /// Mass flow rate.
    MassFlowRate,
    /// Momentum.
    Momentum,
    /// Hydraulic leakage coefficient.
    HydraulicLeakageCoefficient,
    /// Specific heat capacity.
    SpecificHeatCapacity,
    /// Specific gas constant.
    SpecificGasConstant,
    /// Thermal conductance.
    ThermalConductance,
    /// Hydraulic resistance.
    HydraulicResistance,
    /// Pneumatic characteristic impedance.
    PneumaticCharacteristicImpedance,
    /// Turbulent flow coefficient.
    TurbulentFlowCoefficient,
    /// Motor back EMF constant.
    MotorBackEmfConstant,
    /// Screw pitch.
    ScrewPitch,
    /// Thrust-specific fuel consumption.
    ThrustSpecificFuelConsumption,
    /// Volume flow acceleration.
    VolumeFlowAcceleration,
}

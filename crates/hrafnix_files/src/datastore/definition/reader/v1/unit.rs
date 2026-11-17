use hrafnix_datastore::definition::UnitDefinition;
use hrafnix_units::UnitFamilyId;
use serde::{Deserialize, Deserializer};

/// Unit selection expression and family.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UnitReaderV1 {
    /// Item description.
    description: String,
    /// Default expression.
    default: String,
    /// Unit family selected by a plain snake-case string.
    #[serde(deserialize_with = "deserialize_unit_family")]
    unit_family: UnitFamilyId,
}

impl From<UnitReaderV1> for UnitDefinition {
    fn from(unit: UnitReaderV1) -> Self {
        Self::new_with_default(unit.description, unit.unit_family, unit.default)
    }
}

/// Deserializes only plain-string version-1 family identifiers.
fn deserialize_unit_family<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<UnitFamilyId, D::Error> {
    let family = String::deserialize(deserializer)?;
    UnitFamilyReaderV1::deserialize(serde::de::value::StrDeserializer::new(&family))
}

/// Frozen version-1 family identifiers, independent of future runtime additions.
#[derive(Deserialize)]
#[serde(remote = "UnitFamilyId", rename_all = "snake_case")]
enum UnitFamilyReaderV1 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::deserialize::RootObject;
    use serde_json::json;

    #[test]
    fn converts_all_version_one_unit_families() {
        assert_eq!(UnitFamilyId::ALL.len(), 52);
        for family in UnitFamilyId::ALL {
            let name = family
                .description()
                .to_ascii_lowercase()
                .replace([' ', '-'], "_");
            let reader = serde_json::from_value::<RootObject<UnitReaderV1>>(json!({
                "description": "Units",
                "default": "selected_unit",
                "unit_family": name
            }))
            .unwrap();
            assert_eq!(
                UnitDefinition::from(reader.document),
                UnitDefinition::new_with_default("Units", family, "selected_unit"),
                "{name}"
            );
        }
    }

    #[test]
    fn rejects_invalid_unit_families() {
        for family in [
            json!("unknown"),
            json!("Length"),
            json!("length_meter"),
            json!(1),
            json!(["length"]),
            json!({"length": null}),
            serde_json::Value::Null,
        ] {
            assert!(
                serde_json::from_value::<RootObject<UnitReaderV1>>(json!({
                    "description": "",
                    "default": "",
                    "unit_family": family
                }))
                .is_err()
            );
        }
    }
}

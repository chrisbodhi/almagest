# Calculations

## Tether tip speed

When completing calculations using radians, note that radians are actually dimensionless; they are a ratio of the arc length to the radius. Thus, they can be disregarded when ensuring that your units match.

As an example, when calculating the tether tip speed by multiplying the angular velocity by the radius of the skyhook (tether length extending from the rotating center station):


\\[ v_{tip} = ω × r \\]


We know that the tip speed must be in meters (or kilometers) per second, and the radius in meters (or kilometers). The angular velocity ω is given in radians per second, rad/s. We see that the radians are ignored in this calculation in order to get the desired meters (or kilometers) per second.

## Velocity in an orbit

We calculate this based on the body being orbited and the distance from the center of the body to the orbit's altitude.

\\[ v_{orbit} = √(μ/r) \\]

So, for the Earth with a radius of ≈6371 km and a standard gravitational parameter of ≈3.986 × 10^14 m³/s², we get a velocity of about 7.67 km/s for an orbital altitude of 400 km. There's nothing tether-specific here, but we'll need this calculation soon.

## Velocities at highest, lowest positions

We take the velocity of the base station, \\(v_{orbit} \\), and either add or subtract the tether tip speed, depending on the direction of rotation.

At the bottom of the rotation, the tether is traveling in the opposite direction of the base station's orbit.

\\[ v_{lowest} = v_{orbit} - v_{tip} \\]

Conversely, the tether is traveling in the same direction as the base station of the top of the rotation.

\\[ v_{highest} = v_{orbit} + v_{tip} \\]

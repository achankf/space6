import React, { useMemo } from "react";
import Select from "react-select";

import { SelectPlanetAction } from "./action/SelectPlanetAction";

export const PlanetSelector: React.FC<{
  planetNames: string[];
  setSelectPlanet: React.Dispatch<SelectPlanetAction>;
  universeId: number;
  planetId?: number;
}> = ({ planetNames, planetId, setSelectPlanet, universeId }) => {
  const options = useMemo(
    () =>
      planetNames.map((name, index) => ({
        label: name,
        value: index.toString(),
      })),
    [planetNames]
  );

  return (
    <>
      <div>Planet:</div>
      <Select
        options={options}
        value={planetId !== undefined ? options[planetId] : null}
        onChange={(selected) => {
          if (!selected) {
            throw new Error("Unreachable - bad value");
          }

          setSelectPlanet({
            type: "updatePlanet",
            universeId,
            planetId: +selected.value,
          });
        }}
      />
    </>
  );
};

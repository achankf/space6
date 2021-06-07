import React, { useMemo } from "react";
import Select from "react-select";

import { SelectPlanetAction } from "./action/SelectPlanetAction";

export const UniverseSelector: React.FC<{
  universeNames: string[];
  setSelectPlanet: React.Dispatch<SelectPlanetAction>;
  universeId: number;
}> = ({ universeNames, universeId, setSelectPlanet }) => {
  const options = useMemo(
    () =>
      universeNames.map((name, index) => ({
        label: name,
        value: index.toString(),
      })),
    [universeNames]
  );

  return (
    <>
      <div>Universe:</div>
      <Select
        options={options}
        value={options[universeId]}
        onChange={(selected) => {
          if (!selected) {
            throw new Error("Unreachable - bad value");
          }

          setSelectPlanet({
            type: "updateUniverse",
            universeId: +selected.value,
          });
        }}
      />
    </>
  );
};

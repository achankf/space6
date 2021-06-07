import { Reducer } from "react";

import type { Model } from "../../../../pkg";

interface SelectPlanet {
  universeId: number;
  planetId?: number;
  planetNames: string[];
}

export type SelectPlanetAction =
  | {
      type: "updateUniverse";
      universeId: number;
    }
  | {
      type: "updatePlanet";
      planetId: number;
      universeId: number;
    }
  | {
      type: "updateAll";
      planetId: number;
      universeId: number;
    };

export function createSelectPlanetState(
  model: Model,
  universeId: number,
  planetId?: number
): SelectPlanet {
  const planetCount = model.count_planets(universeId);

  const planetNames = (() => {
    const ret = [];

    for (let i = 0; i < planetCount; i++) {
      ret.push(model.get_planet_name(universeId, i));
    }

    return ret;
  })();

  return { universeId, planetId, planetNames };
}

export const createSelectPlanetReducer: (
  model: Model
) => Reducer<SelectPlanet, SelectPlanetAction> = (model) => (state, action) => {
  const { universeId } = action;

  switch (action.type) {
    case "updatePlanet": {
      const { planetId } = action;
      if (universeId !== state.universeId) {
        // can this be possible?
        console.error("Cannot update planet because universeId doesn't match");
        return state; // not updated
      }
      return { ...state, planetId };
    }
    case "updateUniverse": {
      return createSelectPlanetState(model, universeId);
    }
    case "updateAll": {
      const { planetId } = action;
      return createSelectPlanetState(model, universeId, planetId);
    }
    default:
      throw new Error("Unreachable - bad action");
  }
};

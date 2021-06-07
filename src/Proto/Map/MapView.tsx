import React, {
  useContext,
  useEffect,
  useMemo,
  useReducer,
  useState,
} from "react";

import type { PlanetMapFacade } from "../../../pkg";
import { ModelContext } from "../../context/ModelProxy";
import {
  createSelectPlanetReducer,
  createSelectPlanetState,
} from "./action/SelectPlanetAction";
import { MapCanvas } from "./MapCanvas";
import { MapMode } from "./MapMode";
import { MapModeToolbar } from "./MapModeToolbar";
import { PlanetSelector } from "./PlanetSelector";
import { UniverseSelector } from "./UniverseSelector";

export const MapView: React.FC = () => {
  const { model, universeNames } = useContext(ModelContext);
  const [mapMode, setMapMode] = useState(MapMode.Terrain);
  const [isShowGrid, setShowGrid] = useState(false);
  const [planetFacade, setPlanetFacade] = useState<PlanetMapFacade | undefined>(
    undefined
  );
  const selectPlanetReducer = useMemo(
    () => createSelectPlanetReducer(model),
    [model]
  );
  const [{ universeId, planetId, planetNames }, setSelectPlanet] = useReducer(
    selectPlanetReducer,
    createSelectPlanetState(model, 0, 0)
  );
  const tick = model.get_tick();

  useEffect(() => {
    const facade = model.create_planet_facade();
    setPlanetFacade(facade);

    return () => {
      if (planetFacade) {
        planetFacade.free();
        setPlanetFacade(undefined);
      }
    };
  }, [tick]);

  if (!planetFacade) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <div
        style={{
          display: "grid",
          gridTemplateColumns: "auto 1fr",
          alignItems: "center",
        }}
      >
        <UniverseSelector
          setSelectPlanet={setSelectPlanet}
          universeId={universeId}
          universeNames={universeNames}
        />
        <PlanetSelector
          setSelectPlanet={setSelectPlanet}
          planetId={planetId}
          planetNames={planetNames}
          universeId={universeId}
        />
      </div>
      {planetId !== undefined ? (
        <div>
          <div>Planet {planetFacade.get_name()}</div>
          <div>
            Map Modes
            <button onClick={() => setShowGrid(!isShowGrid)}>Grid</button>:
            <MapModeToolbar>
              <button onClick={() => setMapMode(MapMode.Terrain)}>
                Terrain
              </button>
              <button onClick={() => setMapMode(MapMode.Height)}>Height</button>
            </MapModeToolbar>
            <MapCanvas
              planetFacade={planetFacade}
              mapMode={mapMode}
              universeId={universeId}
              planetId={planetId}
              isShowGrid={isShowGrid}
            />
          </div>
        </div>
      ) : null}
    </div>
  );
};

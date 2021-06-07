import React, {
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";
import ReactTooltip from "react-tooltip";

import type { PlanetMapFacade } from "../../../pkg";
import { ModelContext } from "../../context/ModelProxy";
import { CoorCalculator } from "../CoorCalculator";
import { MapMode } from "./MapMode";

interface Props {
  universeId: number;
  planetId: number;
  planetFacade: PlanetMapFacade;
  mapMode: MapMode;
  isShowGrid: boolean;
}

const gridSize = 10;

export const MapCanvas: React.FC<Props> = ({
  universeId,
  planetId,
  planetFacade,
  mapMode,
  isShowGrid,
}) => {
  const { model } = useContext(ModelContext);

  const [state, setState] =
    useState<{ x: number; y: number; regionIndex: number }>();

  const baseMapCanvas = useRef<HTMLCanvasElement>(null);
  const hoverMapCanvas = useRef<HTMLCanvasElement>(null);
  const gridMapCanvas = useRef<HTMLCanvasElement>(null);
  const mapCanvas = useRef<HTMLCanvasElement>(null);

  const draw = useCallback(() => {
    if (
      baseMapCanvas.current &&
      hoverMapCanvas.current &&
      mapCanvas.current &&
      gridMapCanvas.current
    ) {
      const context = mapCanvas.current.getContext("2d");

      if (!context) {
        throw new Error("Cannot get 2D context");
      }

      context.drawImage(baseMapCanvas.current, 0, 0);
      if (isShowGrid) {
        context.drawImage(gridMapCanvas.current, 0, 0);
      }
      context.drawImage(hoverMapCanvas.current, 0, 0);
    }
  }, [baseMapCanvas, hoverMapCanvas, mapCanvas, gridMapCanvas, isShowGrid]);

  useEffect(() => {
    switch (mapMode) {
      case MapMode.Terrain: {
        model.update_base_canvas_with_terrain(universeId, planetId, gridSize);
        break;
      }
      case MapMode.Height: {
        model.update_base_canvas_with_height(universeId, planetId, gridSize);
        break;
      }
      default:
        throw new Error("Unreachable - unsupported map mode");
    }

    model.update_grid_canvas(universeId, planetId, gridSize);

    draw();
  }, [mapMode, isShowGrid]);

  const planetName = planetFacade.get_name();
  const { model_width, model_height } = planetFacade;
  const tooltipId = `TerrainMap ${planetName}`;

  const coorCal = new CoorCalculator(model_width, model_height, {
    topLeft: [0, 0],
    gridSize,
  });

  const vpWidth = coorCal.toVpMagnitude(model_width);
  const vpHeight = coorCal.toVpMagnitude(model_height);

  return (
    <div>
      <canvas
        id="map-hover"
        ref={hoverMapCanvas}
        height={vpHeight}
        width={vpWidth}
        style={{ display: "none" }}
      />
      <canvas
        id="map-base"
        ref={baseMapCanvas}
        height={vpHeight}
        width={vpWidth}
        style={{ display: "none" }}
      />
      <canvas
        id="map-grid"
        ref={gridMapCanvas}
        height={vpHeight}
        width={vpWidth}
        style={{ display: "none" }}
      />
      <canvas
        id="map"
        ref={mapCanvas}
        data-for={tooltipId}
        data-tip
        height={vpHeight}
        width={vpWidth}
        onMouseMove={(e) => {
          const rect = e.currentTarget.getBoundingClientRect();
          const x = e.clientX - rect.left;
          const y = e.clientY - rect.top;
          const [modelX, modelY] = coorCal.toModelCoor([x, y]);
          const regionIndex = model.find_region_index(
            universeId,
            planetId,
            modelX,
            modelY
          );

          model.update_terrain_highlight_canvas(
            universeId,
            planetId,
            gridSize,
            regionIndex
          );

          draw();
          setState({ x, y, regionIndex });
        }}
      />

      <ReactTooltip id={tooltipId}>{JSON.stringify(state)}</ReactTooltip>
    </div>
  );
};

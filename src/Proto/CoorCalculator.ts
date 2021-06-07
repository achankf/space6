export const MIN_GRID_SIZE = 50;
export const MAX_GRID_SIZE = 300;
export const RADIUS = 3 / 10;

export const MAP_CANVAS_ID = "map";

export type Vec2D = [number, number];

export interface ViewportData {
  topLeft: Vec2D;
  gridSize: number;
}

export class CoorCalculator {
  constructor(
    private modelWidth: number,
    private modelHeight: number,
    private vpData: ViewportData
  ) {}

  public centerVp(vpAt: Vec2D): Vec2D {
    const { topLeft } = this.vpData;
    const at = this.toModelCoor(vpAt);
    const offset = subtract(this.getCenter(), at);
    return add(topLeft, offset);
  }

  public toVpCoor([x, y]: Vec2D): Vec2D {
    const { topLeft, gridSize } = this.vpData;
    const [tlX, tlY] = topLeft;
    return [(x + tlX) * gridSize, (y + tlY) * gridSize];
  }

  public toModelCoor([x, y]: Vec2D): Vec2D {
    const { topLeft, gridSize } = this.vpData;
    const [tlX, tlY] = topLeft;
    return [x / gridSize - tlX, y / gridSize - tlY];
  }

  public toVpMagnitude(val: number): number {
    return val * this.vpData.gridSize;
  }

  public getVpCenter(): Vec2D {
    const vpWidth = this.toVpMagnitude(this.modelWidth);
    const vpHeight = this.toVpMagnitude(this.modelHeight);
    return [vpWidth / 2, vpHeight / 2];
  }

  public getCenter(): Vec2D {
    return this.toModelCoor(this.getVpCenter());
  }
}

function subtract([ax, ay]: Vec2D, [bx, by]: Vec2D): Vec2D {
  return [ax - bx, ay - by];
}

function add([ax, ay]: Vec2D, [bx, by]: Vec2D): Vec2D {
  return [ax + bx, ay + by];
}

// Map and Geometry Types
export interface Coordinates {
  lat: number;
  lng: number;
}

export interface MapPosition {
  lat: number;
  lng: number;
  zoom: number;
}

export interface GeoJSONFeature {
  type: "Feature";
  geometry: {
    type: "Point" | "LineString" | "Polygon";
    coordinates: number[] | number[][] | number[][][];
  };
  properties?: Record<string, any>;
}

export interface GeoJSONCollection {
  type: "FeatureCollection";
  features: GeoJSONFeature[];
}

// Transport Line Types
export interface LinePoint {
  id: string;
  lat: number;
  lng: number;
  lineId: string;
  refMarker?: any;
}

export interface TransportLine {
  id: string;
  name: string;
  type: string;
  color: string;
  pointIds: string[];
  customSpeedLimits: Record<string, number>;
  customAcceleration: number;
  customStopTime: number;
  customMaxSpeed: number;
  customCoverage: number;
  lineThickness: number;
}

export interface ParallelLine {
  points: string[];
  count: number;
}

// Store Types
export interface LinesState {
  projectName: string;
  lines: Record<string, TransportLine>;
  parallels: ParallelLine[];
  points: Record<string, LinePoint>;
  lineIdCounter: number;
}

export interface UIState {
  mapStyle: string;
  mapPosition: MapPosition;
  isLoading: boolean;
  error: string | null;
}

export interface PaxState {
  coverageData: any;
  isCalculating: boolean;
  error: string | null;
}

// API Types
export interface StationInfoRequest {
  stations: Station[];
  separation_distance?: number;
  method?: string;
  routing?: string;
}

export interface FindStationRequest {
  stations: Station[];
  route: Coordinates[];
  method?: string;
  routing?: string;
}

export interface Station {
  id: string;
  lat: number;
  lng: number;
  name?: string;
}

export interface OptimalStationResult {
  station: Station;
  score: number;
  coverage: number;
}

// Component Props
export interface MapComponentProps {
  center?: MapPosition;
  zoom?: number;
  style?: string;
}

export interface LineComponentProps {
  line: TransportLine;
  isSelected?: boolean;
  onClick?: (line: TransportLine) => void;
}

// Event Types
export interface MapExportEvent {
  urlData: string;
  heightRatio: number;
}

// Error Types
export interface AppError {
  message: string;
  code?: string;
  details?: any;
}
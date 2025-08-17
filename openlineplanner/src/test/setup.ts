import { config } from "@vue/test-utils";
import { vi } from "vitest";

// Mock mapbox-gl
vi.mock("mapbox-gl", () => ({
  default: vi.fn().mockImplementation(() => ({
    on: vi.fn(),
    off: vi.fn(),
    addSource: vi.fn(),
    addLayer: vi.fn(),
    removeLayer: vi.fn(),
    removeSource: vi.fn(),
    getStyle: vi.fn(() => ({ layers: [] })),
    getCenter: vi.fn(() => ({ lng: 0, lat: 0 })),
    setCenter: vi.fn(),
    setZoom: vi.fn(),
    resize: vi.fn(),
    getCanvas: vi.fn(() => ({ style: {} })),
    dragRotate: { disable: vi.fn() },
    touchZoomRotate: { disableRotation: vi.fn() },
  })),
  accessToken: "",
}));

// Mock html2canvas
vi.mock("html2canvas", () => ({
  default: vi.fn(() => Promise.resolve({
    toDataURL: vi.fn(() => "data:image/png;base64,mock"),
    height: 100,
    width: 100,
  })),
}));

// Global test configuration
config.global.stubs = {
  RouterLink: true,
  RouterView: true,
};
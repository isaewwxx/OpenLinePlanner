import { describe, it, expect, beforeEach, vi } from "vitest";
import { mount } from "@vue/test-utils";
import InteractiveMap from "@/components/InteractiveMap.vue";
import { createPinia, setActivePinia } from "pinia";

// Mock stores
vi.mock("@/stores/editing", () => ({
  useEditStore: () => ({
    stopAllInputs: vi.fn(),
  }),
}));

vi.mock("@/stores/lines", () => ({
  useLinesStore: () => ({
    getLines: [],
    getPoints: [],
    addLine: vi.fn(),
    addPoint: vi.fn(),
  }),
}));

vi.mock("@/stores/pax", () => ({
  usePaxStore: () => ({
    coverageData: null,
    isCalculating: false,
    error: null,
  }),
}));

vi.mock("@/stores/overlay", () => ({
  useOverlayStore: () => ({
    overlays: [],
  }),
}));

vi.mock("@/stores/ui", () => ({
  useUIStore: () => ({
    mapStyle: "mapbox://styles/mapbox/streets-v11",
    mapPosition: { lat: 48.2082, lng: 16.3738, zoom: 12 },
    setMapPosition: vi.fn(),
  }),
}));

describe("InteractiveMap", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("renders correctly", () => {
    const wrapper = mount(InteractiveMap);
    expect(wrapper.find("#map").exists()).toBe(true);
    expect(wrapper.find(".map").exists()).toBe(true);
  });

  it("initializes map on mount", () => {
    const wrapper = mount(InteractiveMap);
    expect(wrapper.vm.map).toBeDefined();
  });

  it("handles map export event", async () => {
    const wrapper = mount(InteractiveMap);
    
    // Simulate export event
    const exportEvent = new CustomEvent("generateMapExport");
    document.dispatchEvent(exportEvent);
    
    // Wait for async operations
    await wrapper.vm.$nextTick();
    
    // Check if print class was added
    expect(wrapper.find(".map").classes()).toContain("print");
  });

  it("updates map position on move", async () => {
    const wrapper = mount(InteractiveMap);
    const uiStore = wrapper.vm.uiStore;
    
    // Simulate map move
    const mockCenter = { lat: 50.0, lng: 10.0 };
    wrapper.vm.map.getCenter = vi.fn().mockReturnValue(mockCenter);
    
    // Trigger moveend event
    wrapper.vm.map.fire("moveend");
    
    await wrapper.vm.$nextTick();
    
    expect(uiStore.setMapPosition).toHaveBeenCalledWith(mockCenter);
  });

  it("handles mouse clicks correctly", () => {
    const wrapper = mount(InteractiveMap);
    const editStore = wrapper.vm.editStore;
    
    // Mock mouse event
    const mockEvent = {
      originalEvent: {
        target: wrapper.vm.map.getCanvas(),
      },
      lngLat: { lng: 16.3738, lat: 48.2082 },
    };
    
    // Simulate mouse click
    wrapper.vm.map.fire("mousedown", mockEvent);
    
    expect(editStore.stopAllInputs).toHaveBeenCalled();
  });

  it("loads state correctly", () => {
    const wrapper = mount(InteractiveMap);
    const linesStore = wrapper.vm.linesStore;
    
    // Mock lines data
    const mockLines = [
      { id: "1", name: "Line 1", color: "#ff0000" },
      { id: "2", name: "Line 2", color: "#00ff00" },
    ];
    
    linesStore.getLines = mockLines;
    
    // Call loadState
    wrapper.vm.loadState();
    
    // Verify lines are loaded
    expect(wrapper.vm.lines).toEqual(mockLines);
  });

  it("handles errors gracefully", async () => {
    const wrapper = mount(InteractiveMap);
    
    // Mock map initialization error
    const consoleSpy = vi.spyOn(console, "error").mockImplementation(() => {});
    
    // Simulate error during map initialization
    wrapper.vm.map = null;
    wrapper.vm.loadState();
    
    await wrapper.vm.$nextTick();
    
    expect(consoleSpy).toHaveBeenCalled();
    consoleSpy.mockRestore();
  });

  it("cleans up resources on unmount", () => {
    const wrapper = mount(InteractiveMap);
    const map = wrapper.vm.map;
    
    // Mock map cleanup methods
    map.off = vi.fn();
    map.remove = vi.fn();
    
    // Unmount component
    wrapper.unmount();
    
    expect(map.off).toHaveBeenCalled();
    expect(map.remove).toHaveBeenCalled();
  });
});
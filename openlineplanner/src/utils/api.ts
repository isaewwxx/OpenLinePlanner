import type { 
  StationInfoRequest, 
  FindStationRequest, 
  OptimalStationResult,
  Station,
  AppError 
} from "@/types";

// API Configuration
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || "http://localhost:8080/api/v1";
const DEFAULT_TIMEOUT = 30000; // 30 seconds

// Custom error class for API errors
export class APIError extends Error {
  public status: number;
  public code: string;
  public details?: any;

  constructor(message: string, status: number, code: string, details?: any) {
    super(message);
    this.name = "APIError";
    this.status = status;
    this.code = code;
    this.details = details;
  }
}

// Request interceptor
const createRequest = async (endpoint: string, options: RequestInit = {}): Promise<RequestInit> => {
  const url = `${API_BASE_URL}${endpoint}`;
  
  const defaultOptions: RequestInit = {
    headers: {
      "Content-Type": "application/json",
      "Accept": "application/json",
    },
    timeout: DEFAULT_TIMEOUT,
  };

  return {
    ...defaultOptions,
    ...options,
    headers: {
      ...defaultOptions.headers,
      ...options.headers,
    },
  };
};

// Response interceptor
const handleResponse = async (response: Response): Promise<any> => {
  if (!response.ok) {
    let errorData: any;
    try {
      errorData = await response.json();
    } catch {
      errorData = { message: "Unknown error occurred" };
    }

    throw new APIError(
      errorData.message || `HTTP ${response.status}`,
      response.status,
      errorData.error || "UNKNOWN_ERROR",
      errorData.details
    );
  }

  const contentType = response.headers.get("content-type");
  if (contentType && contentType.includes("application/json")) {
    return response.json();
  }
  
  return response.text();
};

// Generic API request function
const apiRequest = async <T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> => {
  try {
    const requestOptions = await createRequest(endpoint, options);
    const response = await fetch(`${API_BASE_URL}${endpoint}`, requestOptions);
    return await handleResponse(response);
  } catch (error) {
    if (error instanceof APIError) {
      throw error;
    }
    
    // Network or other errors
    throw new APIError(
      error instanceof Error ? error.message : "Network error",
      0,
      "NETWORK_ERROR"
    );
  }
};

// API endpoints
export const api = {
  // Station information
  async getStationInfo(request: StationInfoRequest): Promise<any> {
    return apiRequest("/station-info", {
      method: "POST",
      body: JSON.stringify(request),
    });
  },

  // Find optimal station
  async findOptimalStation(request: FindStationRequest): Promise<OptimalStationResult> {
    return apiRequest("/find-station", {
      method: "POST",
      body: JSON.stringify(request),
    });
  },

  // Coverage information
  async getCoverageInfo(router: string, request: any): Promise<any> {
    return apiRequest(`/coverage-info/${router}`, {
      method: "POST",
      body: JSON.stringify(request),
    });
  },

  // Layers
  async getLayers(): Promise<any> {
    return apiRequest("/layers");
  },

  // OSM data
  async getOsmData(): Promise<any> {
    return apiRequest("/osm");
  },

  // Health check
  async healthCheck(): Promise<{ status: string; timestamp: string; version: string }> {
    return apiRequest("/health");
  },

  // Readiness check
  async readinessCheck(): Promise<{ status: string; layers_loaded: boolean }> {
    return apiRequest("/ready");
  },
};

// Utility functions
export const apiUtils = {
  // Check if API is available
  async isAvailable(): Promise<boolean> {
    try {
      await api.healthCheck();
      return true;
    } catch {
      return false;
    }
  },

  // Retry function with exponential backoff
  async retry<T>(
    fn: () => Promise<T>,
    maxRetries: number = 3,
    baseDelay: number = 1000
  ): Promise<T> {
    let lastError: Error;
    
    for (let attempt = 0; attempt <= maxRetries; attempt++) {
      try {
        return await fn();
      } catch (error) {
        lastError = error instanceof Error ? error : new Error(String(error));
        
        if (attempt === maxRetries) {
          throw lastError;
        }
        
        // Exponential backoff
        const delay = baseDelay * Math.pow(2, attempt);
        await new Promise(resolve => setTimeout(resolve, delay));
      }
    }
    
    throw lastError!;
  },

  // Debounce function for API calls
  debounce<T extends (...args: any[]) => any>(
    func: T,
    wait: number
  ): (...args: Parameters<T>) => void {
    let timeout: NodeJS.Timeout;
    
    return (...args: Parameters<T>) => {
      clearTimeout(timeout);
      timeout = setTimeout(() => func(...args), wait);
    };
  },
};

// Export types
export type { APIError };
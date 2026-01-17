/**
 * Health check service
 */

import { apiClient } from '@/client/axios-instance';

export interface HealthCheckResponse {
  status: string;
  timestamp: string;
}

/**
 * Get health check status from the control plane
 */
export const getHealthCheck = async (): Promise<HealthCheckResponse> => {
  const response = await apiClient.get<HealthCheckResponse>('/health');
  return response.data;
};

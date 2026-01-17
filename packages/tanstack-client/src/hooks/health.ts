/**
 * Health check hook using TanStack Query
 */

import { useQuery } from '@tanstack/react-query';
import { getHealthCheck, type HealthCheckResponse } from 'api-client';

// Query keys
export const queryKeys = {
  health: {
    check: () => ['health', 'check'] as const,
  },
};

export interface UseHealthCheckReturn {
  data: HealthCheckResponse | undefined;
  isLoading: boolean;
  isError: boolean;
  error: Error | null;
  refetch: () => void;
}

/**
 * Hook to get health check status
 */
export const useHealthCheck = (): UseHealthCheckReturn => {
  const { data, isLoading, isError, error, refetch } = useQuery({
    queryKey: queryKeys.health.check(),
    queryFn: getHealthCheck,
    staleTime: 5 * 60 * 1000, // 5 minutes
  });

  return {
    data,
    isLoading,
    isError,
    error: error as Error | null,
    refetch,
  };
};

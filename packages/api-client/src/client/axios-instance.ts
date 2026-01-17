/**
 * Axios instance configuration
 */

import axios, { type AxiosInstance } from 'axios';
import { envConfig } from '../config/env';

export const createAxiosInstance = (): AxiosInstance => {
  const instance = axios.create({
    baseURL: envConfig.CONTROL_PLANE_URL,
    timeout: 10000,
    headers: {
      'Content-Type': 'application/json',
    },
  });

  return instance;
};

// Default axios instance
export const apiClient = createAxiosInstance();

// Export the instance for consumers who want to customize it
export { axios };

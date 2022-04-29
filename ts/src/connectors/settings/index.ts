import * as fs from 'fs';
import { dirname } from 'path';

export interface Reducer<T> {
  (state: T, action: any): T;
}

export const persistedReducer = <T>(reducer: Reducer<T>, path: string, keys?: string[]): Reducer<T> => {
  return (state: T, action: any): T => {
    const storeDir = dirname(path);
    const newState = reducer(state, action);
    const fieldsToPersist = Object.entries(newState).filter(([key]) => (keys ? keys.includes(key) : true));
    const persistantState = Object.fromEntries(fieldsToPersist);
    fs.mkdirSync(storeDir, { recursive: true });
    fs.writeFileSync(path, JSON.stringify(persistantState));
    return newState;
  };
};

export const restoreState = <T>(state: T, path: string, keys?: string[]): T => {
  try {
    const storedState = JSON.parse(fs.readFileSync(path, { encoding: 'utf8' }) ?? '{}');
    const requestedState = Object.fromEntries(
      Object.entries(storedState).filter(([key]) => keys?.includes(key) ?? false),
    );
    return {
      ...state,
      ...requestedState,
    };
  } catch {
    return { ...state };
  }
};

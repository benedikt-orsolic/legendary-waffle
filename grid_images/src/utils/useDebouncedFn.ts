import React from "react";

/**
 * @param fn debounced function
 * @param timeout debounce time in ms
 */
export default function useDebouncedFn<T>(
  fn: (args: T) => void,
  timeout: number = 250
) {
  const timer = React.useRef<undefined | NodeJS.Timeout>(undefined);

  return (args: T) => {
    clearTimeout(timer.current);
    timer.current = setTimeout(() => {
      fn(args);
    }, timeout);
  };
}

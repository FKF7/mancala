import axios from "axios";

export async function executeRequest<T>(
  route: string,
  options: RequestInit = {},
): Promise<T> {
  const response = await fetch(route, {
    ...options,
    headers: {
      "Content-Type": "application/json",
      ...options.headers,
    },
  });

  if (!response.ok) {
    throw new Error(`Request failed: ${response.status}`);
  }

  return response.json() as Promise<T>;
}

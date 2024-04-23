const DEV_BACKEND_URL = import.meta.env.VITE_BACKEND_ORIGIN
    ? import.meta.env.VITE_BACKEND_ORIGIN
    : "http://localhost:8080";

const BACKEND_URL = import.meta.env.PROD ? "" : DEV_BACKEND_URL;

export interface UrlData {
    code: string;
    url: string;
}
export interface CreateUrlData {
    code?: string;
    url: string;
}

export const createUrl = async (data: CreateUrlData) => {
    const response = await fetch(`${BACKEND_URL}/`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    });
    const json = await response.json();
    return {
        status: response.status,
        data: json.data
    }
};
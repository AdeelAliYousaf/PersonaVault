import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function App() {
  const [response, setResponse] = useState<string>("Loading...");

  useEffect(() => {
    const fetchData = async () => {
      try {
        const res = await invoke<string>("get_data_from_fastapi");
        setResponse(res);
      } catch (err) {
        console.error("Error:", err);
        setResponse("Failed to connect to FastAPI");
      }
    };

    fetchData();
  }, []);

  return (
    <div className="p-8 text-center">
      <h1 className="text-2xl font-bold mb-4">PersonaVault Test</h1>
      <p className="text-lg">{response}</p>
    </div>
  );
}

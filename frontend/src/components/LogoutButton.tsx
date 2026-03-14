import { API_BASE_URL } from "../App";
import { useState } from "react";

type Props = {
  onClick: () => void;
};

export default function LogoutButton({onClick}:Props) {
    const [loading, loadingSet] = useState(false)
    const logout = async () => {
        loadingSet(true);
        try {
            const res = await fetch(`${API_BASE_URL}/logout`, {
                method: "POST",
                credentials: "include"
            });
            if (!res.ok) {
                const msg = await res.text();
                console.error(msg);
                return;
            }
            console.log(await res.text());
        } catch {
            console.error("Failed to connect to server");
        } finally {
            loadingSet(false);
        }
    }
    return (
        <button onClick={async ()=>{await logout();await onClick();}} disabled={loading}>{loading ? "Logging out..." : "Logout"}</button>
    );
}
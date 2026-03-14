import { useState } from "react";
import { Link } from "react-router-dom";


export default function Signup() {

    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [uid, setUid] = useState("");
    const [error,setError] = useState("");

    const signup = async () => {

        const res = await fetch("http://localhost:3000/signup", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ username, password })
        });
        
        if(!res.ok){
            const msg = await res.text();
            setError(msg);
            return;
        }

        const data = await res.json();

        setUid(data.uid);
        setError("");
    }

    return (
        <div>

            <h1>Signup</h1>

            <input
                placeholder="username"
                onChange={e => setUsername(e.target.value)}
            />

            <input
                placeholder="password"
                onChange={e => setPassword(e.target.value)}
            />

            <button onClick={signup}>
                create account
            </button>

            {error && <p style={{color:"red"}}>{error}</p>}
            <p>UID: {uid}</p>
            <p>
                <Link to="/login">
                    <button>Go to Login</button>
                </Link>
            </p>


        </div>
    );
}
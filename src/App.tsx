import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div id="outBorder">
      <ul>
        <li>
          <ul>
            <li></li>
            <li></li>
            <li></li>
            <li>
              <span className="xiaxie"></span>
            </li>
            <li>
              <span className="shangxie"></span>
            </li>
            <li></li>
            <li></li>
            <li></li>
          </ul>
        </li>
        <li>
          <ul>
            <li>
              <span className="bottomright"></span>
            </li>
            <li>
              <span className="bottomleft"></span>
            </li>
            <li></li>
            <li>
              <span className="shangxie"></span>
            </li>
            <li>
              <span className="xiaxie"></span>
            </li>
            <li></li>
            <li>
              <span className="bottomright"></span>
            </li>
            <li>
              <span className="bottomleft"></span>
            </li>
          </ul>
        </li>
        <li>
          <ul>
            <li>
              <span className="topright"></span>
              <span className="bottomleft"></span>
            </li>
            <li>
              <span className="bottomright"></span>
              <span className="topleft"></span>
            </li>
            <li>
              <span className="bottomleft"></span>
            </li>
            <li>
              <span className="bottomright"></span>
            </li>
            <li>
              <span className="bottomleft"></span>
            </li>
            <li>
              <span className="bottomright"></span>
            </li>
            <li>
              <span className="bottomleft"></span>
              <span className="topright"></span>
            </li>
            <li>
              <span className="bottomright"></span>
              <span className="topleft"></span>
            </li>
          </ul>
        </li>
        <li>
          <ul>
            <li>
              <span className="topleft"></span>
            </li>
            <li>
              <span className="topright"></span>
            </li>
            <li>
              <span className="topleft"></span>
            </li>
            <li>
              <span className="topright"></span>
            </li>
            <li>
              <span className="topleft"></span>
            </li>
            <li>
              <span className="topright"></span>
            </li>
            <li>
              <span className="topleft"></span>
            </li>
            <li>
              <span className="topright"></span>
            </li>
          </ul>
        </li>
        <li id="hejie">
          <ul>
            <li></li>
            <li></li>
            <li></li>
            <li></li>
            <li></li>
            <li></li>
            <li></li>
            <li></li>
          </ul>
          <span id="chuhe">楚河</span>
          <span id="hanjie">界汉</span>
        </li>
        <li>
          <ul>
            <li>
              <span className="bottomleft"></span>
            </li>
            <li>
              <span className="bottomright"></span>
            </li>
            <li>
              <span className="bottomleft"></span>
            </li>
            <li>
              <span className="bottomright"></span>
            </li>
            <li>
              <span className="bottomleft"></span>
            </li>
            <li>
              <span className="bottomright"></span>
            </li>
            <li>
              <span className="bottomleft"></span>
            </li>
            <li>
              <span className="bottomright"></span>
            </li>
          </ul>
        </li>
        <li>
          <ul>
            <li>
              <span className="topleft"></span>
              <span className="bottomright"></span>
            </li>
            <li>
              <span className="topright"></span>
              <span className="bottomleft"></span>
            </li>
            <li>
              <span className="topleft"></span>
            </li>
            <li>
              <span className="topright"></span>
            </li>
            <li>
              <span className="topleft"></span>
            </li>
            <li>
              <span className="topright"></span>
            </li>
            <li>
              <span className="topleft"></span>
              <span className="bottomright"></span>
            </li>
            <li>
              <span className="topright"></span>
              <span className="bottomleft"></span>
            </li>
          </ul>
        </li>
        <li>
          <ul>
            <li>
              <span className="topright"></span>
            </li>
            <li>
              <span className="topleft"></span>
            </li>
            <li></li>
            <li>
              <span className="xiaxie"></span>
            </li>
            <li>
              <span className="shangxie"></span>
            </li>
            <li></li>
            <li>
              <span className="topright"></span>
            </li>
            <li>
              <span className="topleft"></span>
            </li>
          </ul>
        </li>
        <li>
          <ul>
            <li></li>
            <li></li>
            <li></li>
            <li>
              <span className="shangxie"></span>
            </li>
            <li>
              <span className="xiaxie"></span>
            </li>
            <li></li>
            <li></li>
            <li></li>
          </ul>
        </li>
      </ul>
      {/* <div id="qihe">
        <div className="chessPieces black">
          <div>
            <span>將</span>
          </div>
        </div>
        <div className="chessPieces black">
          <div>
            <span>士</span>
          </div>
        </div>
        <div className="chessPieces black">
          <div>
            <span>象</span>
          </div>
        </div>
        <div className="chessPieces black">
          <div>
            <span>馬</span>
          </div>
        </div>
        <div className="chessPieces black">
          <div>
            <span>車</span>
          </div>
        </div>
        <div className="chessPieces black">
          <div>
            <span>砲</span>
          </div>
        </div>
        <div className="chessPieces black">
          <div>
            <span>卒</span>
          </div>
        </div>

        <div className="chessPieces red">
          <div>
            <span>帥</span>
          </div>
        </div>
        <div className="chessPieces red">
          <div>
            <span>仕</span>
          </div>
        </div>
        <div className="chessPieces red">
          <div>
            <span>相</span>
          </div>
        </div>

        <div className="chessPieces red">
          <div>
            <span>傌</span>
          </div>
        </div>
        <div className="chessPieces red">
          <div>
            <span>俥</span>
          </div>
        </div>
        <div className="chessPieces red">
          <div>
            <span>炮</span>
          </div>
        </div>
        <div className="chessPieces red">
          <div>
            <span>兵</span>
          </div>
        </div>
      </div> */}
    </div>
  );
}

export default App;

import React from "react";
import logo from "./logo.svg";
import "./App.css";
import Home from "./pages/home/Home";
import { FavoritesImagesCtxProvider } from "./state/FavoriteImgsState";
function App() {
  return (
    <FavoritesImagesCtxProvider>
      <div className="App">
        <Home />
      </div>
    </FavoritesImagesCtxProvider>
  );
}

export default App;

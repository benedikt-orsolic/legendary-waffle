import React from "react";

const FavoritesImagesCtx = React.createContext<{
  favorites: number[];
  toggleFavId: (id: number) => void;
}>({
  favorites: [] as number[],
  toggleFavId: (id: number) => {},
});

export function FavoritesImagesCtxProvider(props: {
  children: React.ReactNode;
}) {
  const [favorites, setFavorites] = React.useState<number[]>([]);

  return (
    <FavoritesImagesCtx.Provider
      value={{
        favorites,
        toggleFavId: (toggleFavId: number) => {
          const favIdIdx = favorites.findIndex((id) => toggleFavId === id);
          if (favIdIdx === -1) {
            setFavorites([...favorites, toggleFavId]);
            return;
          }

          setFavorites([...favorites.splice(favIdIdx, 1)]);
        },
      }}
    >
      {props.children}
    </FavoritesImagesCtx.Provider>
  );
}

export function useFavoritesImgs() {
  return React.useContext(FavoritesImagesCtx);
}

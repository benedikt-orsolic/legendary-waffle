import React from "react";

import ImgService, { TResult } from "../../services/ImgService";
import ImgGrid from "../../components/img_grid/ImgGrid";
import useDebouncedFn from "../../utils/useDebouncedFn";
import { useFavoritesImgs } from "../../state/FavoriteImgsState";
import LikedViewSelectorBtn from "./components/liked-view-selector-btn/LikedViewSelectorBtn";

export default function Home() {
  const [rawImgs, setRawImgs] = React.useState<TResult | null>(null);
  const debounceSearch = useDebouncedFn(async (searchValue: string) => {
    const data = await ImgService.serachImages({ q: searchValue });
    setRawImgs(data);
  });

  const [isLikedImgs, setIsLikedImgs] = React.useState(false);
  const { favorites } = useFavoritesImgs();

  return (
    <>
      <nav className="home-page__nav">
        <input
          type="text"
          placeholder="Search"
          name="search"
          className="home-page__search-input"
          onChange={(e) => {
            setIsLikedImgs(false);
            debounceSearch(e.target.value);
          }}
        />
        <LikedViewSelectorBtn
          onClick={() => setIsLikedImgs(true)}
          likedImgsCount={favorites.length}
        />
      </nav>
      {rawImgs != null && <ImgGrid pageData={rawImgs} />}
    </>
  );
}

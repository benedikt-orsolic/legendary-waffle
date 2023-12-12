import React from "react";
import ImgService, { TResult } from "../../../../../../services/ImgService";
import { useFavoritesImgs } from "../../../../../../state/FavoriteImgsState";

export default function Ovarlay(props: { imgHit: TResult["hits"][0] }) {
  const [isExpanded, setIsExpanded] = React.useState(false);
  const { favorites, toggleFavId } = useFavoritesImgs();

  return (
    <div className="img-grid__article__focus-overlay">
      <button
        className="img-gird__article__expand"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          className=""
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M3.75 3.75v4.5m0-4.5h4.5m-4.5 0L9 9M3.75 20.25v-4.5m0 4.5h4.5m-4.5 0L9 15M20.25 3.75h-4.5m4.5 0v4.5m0-4.5L15 9m5.25 11.25h-4.5m4.5 0v-4.5m0 4.5L15 15"
          />
        </svg>
      </button>

      <button
        className="img-gird__article__heart"
        onClick={() => {
          // ImgService.toggleFavImgId(props.imgHit.id);
          toggleFavId(props.imgHit.id);
        }}
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill={favorites.includes(props.imgHit.id) ? "red" : "none"}
          viewBox="0 0 24 24"
          strokeWidth={1.5}
          stroke="currentColor"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z"
          />
        </svg>
      </button>
    </div>
  );
}

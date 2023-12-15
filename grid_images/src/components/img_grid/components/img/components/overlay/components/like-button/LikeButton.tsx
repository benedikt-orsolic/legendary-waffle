import { useFavoritesImgs } from "../../../../../../../../state/FavoriteImgsState";
import HeartButton from "../../../../../../../buttons/heart-button/HeartButton";

export default function LikeButton(props: { imgId: number }) {
  const { favorites, toggleFavId } = useFavoritesImgs();
  return (
    <HeartButton
      onClick={() => {
        toggleFavId(props.imgId);
      }}
      buttonClassName="img-gird__article__heart"
      svgFill={favorites.includes(props.imgId) ? "red" : "gray"}
      svgClassName="button--svg--icon"
    />
  );
}

import HeartButton from "../../../../components//buttons/heart-button/HeartButton";

export default function LikedViewSelectorBtn(props: {

likedImgsCount: number;
onClick: () => void;
}) {
  return (
    <div className="home-page__liked-imgs-button">
      <HeartButton
        onClick={props.onClick}
        buttonClassName="home-page__liked-imgs-button-btn"
        svgClassName="button--svg--icon "
      />
      <div className="home-page__liked-imgs-button-counter">{props.likedImgsCount}</div>
    </div>
  );
}

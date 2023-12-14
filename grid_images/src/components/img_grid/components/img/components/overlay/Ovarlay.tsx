import React from "react";
import { TResult } from "../../../../../../services/ImgService";
import LikeButton from "./components/like-button/LikeButton";
import OpenModalButton from "./components//open-mobal-button/OpenModalButton";
import ImgDialog from "./components/img-dialog/ImgDialog";

export default function Ovarlay(props: { imgHit: TResult["hits"][0] }) {
  const [isExpanded, setIsExpanded] = React.useState(false);
  return (
    <>
      <div className="img-grid__article__focus-overlay">
        <OpenModalButton toggleExpand={() => setIsExpanded(!isExpanded)} />
        <LikeButton imgId={props.imgHit.id} />
      </div>
      {isExpanded && (
        <ImgDialog
          onDilogClose={() => setIsExpanded(false)}
          imgHit={props.imgHit}
        />
      )}
    </>
  );
}

import CloseButton from "./components/close-button/CloseButton";
import { TResult } from "../../../../../../../../services/ImgService";

export default function ImgDialog(props: {
  onDilogClose: () => void;
  imgHit: TResult["hits"][0];
}) {

console.log(props.imgHit.imageURL);
  return (
    <div className="img-dialog">
      <CloseButton onClose={props.onDilogClose} />
      <img
        className="img-dilog__img"
        src={props.imgHit.largeImageURL}
        height={props.imgHit.imageHeight}
        width={props.imgHit.imageWidth}
      />
    </div>
  );
}

import { TResult } from "../../../services/ImgService";
import Ovarlay from './components/Ovarlay';

export default function Img(props: { imgHit: TResult["hits"][0] }) {
  const webFormatUrlSegments = props.imgHit.webformatURL.split("_640");
  const baseWebFormatUrl = webFormatUrlSegments[0];
  const webFormatExtention = webFormatUrlSegments[1].split(".")[1];
  console.log(webFormatExtention);
  const webFormatSizeRation =
    props.imgHit.webformatWidth / props.imgHit.webformatHeight;

  const sourcSets = [180, 340, 640, 960].map((size) => {
    const imgItem = {
      url: baseWebFormatUrl + "_" + size + "." + webFormatExtention,
      width: 0,
      height: 0,
    };
    if (webFormatSizeRation < 1) {
      imgItem.width = size;
      imgItem.height = Math.round(size * webFormatSizeRation);

      return imgItem;
    }

    imgItem.height = size;
    imgItem.width = Math.round(size * webFormatSizeRation);
    return imgItem;
  });

  return (
    <article className="img-grid__article ">
      <picture className="img-grid__article__picture">
        {sourcSets.map((set) => (
          <source height="auto" width={set?.width} srcSet={set.url} />
        ))}
        <img className="img-grid__article__img" src={props.imgHit.previewURL} />
      </picture>
      <Ovarlay imgHit={props.imgHit} />
    </article>
  );
}


import { TResult } from "../../services/ImgService";

export default function ImgGrid(props: { pageData: TResult }) {
  return (
    <div>
      {props.pageData.hits.map((imgHit) => (
        <Img imgHit={imgHit} />
      ))}
    </div>
  );
}

function Img(props: { imgHit: TResult["hits"][0] }) {
  const baseWebFormatUrl = props.imgHit.webformatURL.split("_640.jpg")[0];
  const webFormatSizeRation =
    props.imgHit.webformatWidth / props.imgHit.webformatHeight;

  const sourcSets = [180, 340, 640, 960].map((size) => {
    const imgItem = {
      url: baseWebFormatUrl + "_" + size + ".jpg",
      width: 0,
      height: 0,
    };
    if (webFormatSizeRation <= 1) {
      imgItem.width = size;
      imgItem.height = Math.round(size * webFormatSizeRation);

      return imgItem;
    }

    imgItem.height = size;
    imgItem.width = Math.round(size * webFormatSizeRation);
    return imgItem;
  });

  return (
    <article style={{ height: "16rem" }}>
      <picture>
        {sourcSets.map((set) => (
          <source height={set?.height} width={set?.width} srcSet={set.url} />
        ))}
        <img src={props.imgHit.previewURL} width={props.imgHit.previewWidth} />
      </picture>
      <header style={{ height: "4rem" }}>{props.imgHit.tags}</header>
    </article>
  );
}

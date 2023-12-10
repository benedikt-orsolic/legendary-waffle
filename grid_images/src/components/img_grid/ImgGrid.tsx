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
  return (
    <article style={{height: "16rem"}}>
      <img
        src={props.imgHit.previewURL}
	height="auto"
        // height={props.imgHit.previewHeight}
        // width={props.imgHit.previewWidth}
      />
      <header style={{height: "4rem"}}>{props.imgHit.tags}</header>
    </article>
  );
}

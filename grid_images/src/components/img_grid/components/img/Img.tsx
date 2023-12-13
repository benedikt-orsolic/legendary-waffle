import { TResult } from "../../../../services/ImgService";
import Ovarlay from "./components/overlay/Ovarlay";
import useGridImgRatio from "./useGridImgRatio";
import useGetImgSourcSet from "./useGetImgSrcSet";

export default function Img(props: { imgHit: TResult["hits"][0] }) {
  const { heightFraction, widthFraction } = useGridImgRatio(props.imgHit);
  const sourcSets = useGetImgSourcSet(props.imgHit);

  return (
    <article
      className="img-grid__article"
      style={{
        gridRow: `span ${heightFraction}`,
        gridColumn: `span ${widthFraction}`,
      }}
    >
      <div className="img-grid__article__picture__wrapper">
        <img className="img-grid__article__img" srcSet={sourcSets} />
      </div>
      <Ovarlay imgHit={props.imgHit} />
    </article>
  );
}

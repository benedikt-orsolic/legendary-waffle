import { TResult } from "../../services/ImgService";
import Img from './components/img/Img';

export default function ImgGrid(props: { pageData: TResult }) {
  return (
    <div className="img-grid__grid">
      {props.pageData.hits.map((imgHit) => (
        <Img imgHit={imgHit} />
      ))}
    </div>
  );
}


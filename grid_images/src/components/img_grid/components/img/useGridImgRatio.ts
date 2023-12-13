import { TResult } from "../../../../services/ImgService";

export default function useGridImgRation(imgHit: TResult["hits"][0]) {
  let widthFraction = 1;
  let heightFraction = 1;

  if (imgHit.webformatWidth > imgHit.webformatHeight) {
    heightFraction = 1;
    widthFraction = Math.round(imgHit.webformatWidth / imgHit.webformatHeight);
  } else if (imgHit.webformatWidth < imgHit.webformatHeight) {
    widthFraction = 1;
    heightFraction = Math.round(imgHit.webformatHeight / imgHit.webformatWidth);
  }

  if (imgHit.likes > 3000) {
    widthFraction *= 2;
    heightFraction *= 2;
  }

  return { widthFraction, heightFraction };
}

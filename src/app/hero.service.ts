import { Injectable } from '@angular/core';

import { Hero } from './hero';
import { HEROS } from './mock-heroes';

@Injectable({
  providedIn: 'root'
})
export class HeroService {

  constructor() { }

  getHeroes(): Hero[] {
    return HEROS;
  }
}

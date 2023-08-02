import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class MessagesService {

  messages: String[] = [];

  add(message: String): void{
    this.messages.push(message);
  }

  clear(): void{
    this.messages = [];
  }
  
  constructor() { }
}

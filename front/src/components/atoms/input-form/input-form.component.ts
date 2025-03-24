import {Component, EventEmitter, Input, Output} from '@angular/core';

@Component({
  selector: 'app-input-form',
  templateUrl: './input-form.component.html',
  styleUrl: './input-form.component.css'
})
export class InputFormComponent {
  @Input() title!: string;
  @Input() placeholder!: string;
  @Input() secret: boolean = false;
  @Output() valueChange = new EventEmitter<string>();

  onInput(event: Event) {
    const inputValue = (event.target as HTMLInputElement).value;
    this.valueChange.emit(inputValue);
  }
}

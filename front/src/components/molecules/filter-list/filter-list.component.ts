import {Component, EventEmitter, Input, Output} from '@angular/core';
import {NgForOf} from '@angular/common';

@Component({
  selector: 'app-filter-list',
  imports: [
    NgForOf
  ],
  templateUrl: './filter-list.component.html',
  styleUrl: './filter-list.component.css'
})
export class FilterListComponent {
  @Input() title: string = "";
  @Input() subtitle: string = "";
  @Input() elements: string[] = [];
  @Input() width: number = 13;

  @Output() selectedValues = new EventEmitter<string[]>();

  selectedItems: string[] = [];

  onCheckboxChange() {
    this.selectedValues.emit(this.selectedItems);
  }

  onCheckboxToggle(item: string, event: any) {
    if (event.target.checked) {
      this.selectedItems.push(item);
    } else {
      const index = this.selectedItems.indexOf(item);
      if (index !== -1) {
        this.selectedItems.splice(index, 1);
      }
    }

    this.onCheckboxChange();
  }
}

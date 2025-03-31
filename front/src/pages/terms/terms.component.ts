import { Component } from '@angular/core';
import {HomeHeaderComponent} from '../../components/molecules/home-header/home-header.component';
import {HomeFooterComponent} from '../../components/molecules/home-footer/home-footer.component';

@Component({
  selector: 'app-terms',
  imports: [HomeHeaderComponent, HomeFooterComponent],
  templateUrl: './terms.component.html',
  styleUrl: './terms.component.css'
})
export class TermsComponent {

}

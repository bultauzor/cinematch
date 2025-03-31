import { Component } from '@angular/core';
import {HomeHeaderComponent} from '../../components/molecules/home-header/home-header.component';
import {HomeFooterComponent} from '../../components/molecules/home-footer/home-footer.component';

@Component({
  selector: 'app-about',
  imports: [HomeHeaderComponent, HomeFooterComponent],
  templateUrl: './about.component.html',
  styleUrl: './about.component.css'
})
export class AboutComponent {}

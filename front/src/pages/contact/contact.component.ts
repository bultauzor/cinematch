import { Component } from '@angular/core';
import {HomeHeaderComponent} from '../../components/molecules/home-header/home-header.component';
import {HomeFooterComponent} from '../../components/molecules/home-footer/home-footer.component';
import {environment} from '../../environments/environment';

@Component({
  selector: 'app-contact',
  imports: [HomeHeaderComponent,
  HomeFooterComponent],
  templateUrl: './contact.component.html',
  styleUrl: './contact.component.css'
})
export class ContactComponent {}

(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["SirenItem"],{"5bf3":function(e,t,n){},"8aae":function(e,t,n){"use strict";var s=n("5bf3"),a=n.n(s);a.a},"8eeb":function(e,t,n){"use strict";n.r(t);var s=function(){var e=this,t=e.$createElement,n=e._self._c||t;return n("div",{staticClass:"container mw768"},[n("div",{staticClass:"columns"},[n("div",{staticClass:"column is-half"},[n("bulma-input",{attrs:{label:"",placeholder:"Серийный номер",iconLeft:"tag"},model:{value:e.siren.num_pass,callback:function(t){e.$set(e.siren,"num_pass",t)},expression:"siren.num_pass"}})],1),n("div",{staticClass:"column is-half"},[n("bulma-select",{attrs:{list:e.siren_types,"selected-item":e.siren.siren_type,label:"Тип сирены","item-name":"siren_type",iconLeft:"tag"},on:{select:e.onSelect}})],1)]),n("bulma-input",{attrs:{label:"",placeholder:"Адрес",iconLeft:"address-card"},model:{value:e.siren.address,callback:function(t){e.$set(e.siren,"address",t)},expression:"siren.address"}}),n("bulma-select",{attrs:{list:e.contacts,"selected-item":e.siren.contact,label:"Контактное лицо","item-name":"contact",iconLeft:"user"},on:{select:e.onSelect}}),n("bulma-select",{attrs:{list:e.companys,"selected-item":e.siren.company,label:"Организация","item-name":"company",iconLeft:"building"},on:{select:e.onSelect}}),n("div",{staticClass:"columns"},[n("div",{staticClass:"column is-half"},[n("bulma-input",{attrs:{label:"",placeholder:"Широта",iconLeft:"tag"},model:{value:e.siren.latitude,callback:function(t){e.$set(e.siren,"latitude",t)},expression:"siren.latitude"}})],1),n("div",{staticClass:"column is-half"},[n("bulma-input",{attrs:{label:"",placeholder:"Долгота",iconLeft:"tag"},model:{value:e.siren.longitude,callback:function(t){e.$set(e.siren,"longitude",t)},expression:"siren.longitude"}})],1)]),n("bulma-input",{attrs:{label:"",placeholder:"Заметка",iconLeft:"comment"},model:{value:e.siren.note,callback:function(t){e.$set(e.siren,"note",t)},expression:"siren.note"}}),n("div",{staticClass:"field is-grouped is-grouped-centered"},[n("div",{staticClass:"control"},[n("bulma-button",{attrs:{text:"Сохранить",color:"primary"},on:{click:e.submit}})],1),n("div",{staticClass:"control"},[n("bulma-button",{attrs:{text:"Закрыть"},on:{click:e.close}})],1),n("div",{staticClass:"control"},[n("bulma-button",{attrs:{text:"Удалить",color:"danger",onclick:"return confirm('Вы действительно хотите удалить эту запись?');"}})],1)])],1)},a=[],i=n("e653"),l=n("e04f"),c=n("ce02"),o=n("7234"),r={id:0,num_id:0,num_pass:"",siren_type_id:"",siren_type:o["a"],address:"",radio:"",desk:"",contact_id:0,contact:o["a"],company_id:0,company:o["a"],latitude:"",longitude:"",stage:"",own:"",note:""},u=n("0036"),d=n("2af4"),m={name:"SirenItem",components:{"bulma-button":i["a"],"bulma-input":l["a"],"bulma-select":c["a"]},mixins:[d["a"],u["a"]],data:function(){return{title:"",siren:r,siren_types:[o["a"]],contacts:[o["a"]],companys:[o["a"]]}},mounted:function(){this.fetchItem("siren","Siren",[],["siren_type","contact","company"],[])},methods:{submit:function(){this.submitItem("siren","Siren",[],[])},onSelect:function(e,t){this.siren[t]=e,this.siren["".concat(t,"_id")]=e.id},delete:function(){}}},p=m,b=(n("8aae"),n("2877")),f=Object(b["a"])(p,s,a,!1,null,"3cdb719c",null);t["default"]=f.exports}}]);
//# sourceMappingURL=SirenItem-legacy.7b3891ad.js.map
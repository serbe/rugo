(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["CompanyItem"],{"0147":function(e,a,t){"use strict";t.r(a);var n=function(){var e=this,a=e.$createElement,t=e._self._c||a;return t("div",{staticClass:"container mw768"},[t("bulma-input",{attrs:{label:"",placeholder:"Наименование организации",iconLeft:"building"},model:{value:e.company.name,callback:function(a){e.$set(e.company,"name",a)},expression:"company.name"}}),t("bulma-select",{attrs:{list:e.scopes,"selected-item":e.company.scope,"item-name":"scope",label:"Сфера деятельности",iconLeft:"tag"},on:{select:e.onSelect}}),t("bulma-input",{attrs:{label:"",placeholder:"Адрес",iconLeft:"address-card"},model:{value:e.company.address,callback:function(a){e.$set(e.company,"address",a)},expression:"company.address"}}),t("div",{staticClass:"columns"},[t("div",{staticClass:"column"},[t("div",{staticClass:"field"},[t("label",{staticClass:"label"},[e._v("Электронный адрес")]),e._l(e.company.emails,function(a,n){return t("bulma-input",{key:n,attrs:{type:"email",placeholder:"Электронный адрес",iconLeft:"envelope",autocomplete:"email",pattern:"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\\.[a-zA-Z0-9-]+)*$",error:"Неправильный email"},on:{blur:function(a){return e.onBlur("emails","email")}},model:{value:e.company.emails[n],callback:function(a){e.$set(e.company.emails,n,a)},expression:"company.emails[index]"}})})],2)]),t("div",{staticClass:"column"},[t("div",{staticClass:"field"},[t("label",{staticClass:"label"},[e._v("Телефон")]),e._l(e.company.phones,function(a,n){return t("bulma-input",{key:n,attrs:{type:"tel",placeholder:"Телефон",iconLeft:"phone",autocomplete:"tel"},on:{blur:function(a){return e.onBlur("phones","phone")}},model:{value:e.company.phones[n],callback:function(a){e.$set(e.company.phones,n,a)},expression:"company.phones[index]"}})})],2)]),t("div",{staticClass:"column"},[t("div",{staticClass:"field"},[t("label",{staticClass:"label"},[e._v("Факс")]),e._l(e.company.faxes,function(a,n){return t("bulma-input",{key:n,attrs:{type:"tel",placeholder:"Факс",iconLeft:"phone",autocomplete:"tel"},on:{blur:function(a){return e.onBlur("faxes","phone")}},model:{value:e.company.faxes[n],callback:function(a){e.$set(e.company.faxes,n,a)},expression:"company.faxes[index]"}})})],2)])]),e.company.practices.length>0?t("div",{key:"practices",staticClass:"field"},[t("label",{staticClass:"label"},[e._v("Тренировки")]),e._l(e.company.practices,function(e){return t("bulma-input",{key:e.id,attrs:{value:e.date_str+" - "+e.kind_name+" - "+e.topic,hyper:"/practice/"+e.id,iconLeft:"history",readonly:""}})})],2):e._e(),e.company.contacts.length>0?t("div",{key:"contacts",staticClass:"field"},[t("label",{staticClass:"label"},[e._v("Сотрудники")]),e._l(e.company.contacts,function(e){return t("bulma-input",{key:e.id,attrs:{value:e.name+" - "+e.post_name,hyper:"/contact/"+e.id,iconLeft:"user",readonly:""}})})],2):e._e(),t("bulma-input",{attrs:{label:"",placeholder:"Заметка",iconLeft:"sticky-note"},model:{value:e.company.note,callback:function(a){e.$set(e.company,"note",a)},expression:"company.note"}}),t("div",{staticClass:"field is-grouped is-grouped-centered"},[t("div",{staticClass:"control"},[t("bulma-button",{attrs:{text:"Сохранить",color:"primary"},on:{click:e.submit}})],1),t("div",{staticClass:"control"},[t("bulma-button",{attrs:{text:"Закрыть"},on:{"~click":function(a){return e.close(a)}}})],1),t("div",{staticClass:"control"},[t("bulma-button",{attrs:{text:"Удалить",color:"danger",onclick:"return confirm('Вы действительно хотите удалить эту запись?');"}})],1)])],1)},s=[],l=t("e653"),o=t("e04f"),c=t("ce02"),i=t("7234"),m={id:0,name:"",address:"",scope:i["a"],scope_id:0,note:"",emails:[],phones:[],faxes:[],practices:[{id:0,date_str:"",kind_name:"",topic:""}],contacts:[{id:0,name:"",department_name:"",post_name:"",post_go_name:""}]},p=t("0036"),r=t("2af4"),u={name:"CompanyItem",components:{"bulma-button":l["a"],"bulma-input":o["a"],"bulma-select":c["a"]},mixins:[r["a"],p["a"]],data(){return{title:"",company:m,scopes:[i["a"]]}},mounted(){this.fetchItem("company","Company",["emails","phones","faxes"],["scope"],[["practice","PracticeList"]])},methods:{onBlur(e){this.company[e]=this.checkArray(this.company[e])},onSelect(e){this.scope=e,this.company.scope_id=e.id},submit(){this.submitItem("company","Company",["emails"],["phones","faxes"])},delete(){},customLabel(e){return e.name}}},d=u,y=t("2877"),b=Object(y["a"])(d,n,s,!1,null,null,null);a["default"]=b.exports}}]);
//# sourceMappingURL=CompanyItem.9e798404.js.map
enum TrafficLight{                             
   RED,                        
   GREEN,                           
   YELLOW,
}
fn value_in_light(light:TrafficLight)->u8{
   match light{
    TrafficLight::RED=>20,
    TrafficLight::GREEN=>30,
    TrafficLight::YELLOW=>3,
 } 
}
fn main(){
    println!("{}",value_in_light(TrafficLight::RED));
    println!("{}",value_in_light(TrafficLight::GREEN));
    println!("{}",value_in_light(TrafficLight::YELLOW));
 }
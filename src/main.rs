use std::vec;
fn main(){
    let mut data = String::new();
    let mut num1:usize=0;
    let mut num2:usize=0;
    let mut flag:bool=false;
    //let v = vec![1, 2, 3];
    println!("请输入日期:");
    let _ =std::io::stdin().read_line(&mut data);
    println!("日期{}", data);
    for(index,c) in data.char_indices()
    {
        if c=='.'
        {
            if num1==0
            {
                num1=index;
            }
            else if num2==0
            {
                num2=index;
            }
            else {
                flag=true
            }
        }
        if flag==true
        {
            break;
        }
    }
    let year=&data[0..num1];
    let month=&data[num1+1..num2];
    let date=&data[num2+1..];
    println!("{}|{}|{}\n", year,month,date);
    let year_n=year.trim().parse::<i32>().unwrap();
    let month_n=month.trim().parse::<i32>().unwrap();
    let date_n=date.trim().parse::<i32>().unwrap();
    let flag=is_valid(year_n, month_n, date_n);
    if flag==true
    {
        println!("Valid date");
    }
    else {
        println!("Invalid date");
    }
 }
 fn is_valid(year_n:i32,month_n:i32,date_n:i32)->bool
 {
    let days_vec = vec![31,28,31,30,31,30,31,31,30,31,30,31];
    let m=month_n as usize;
    if month_n<0||month_n>12
    {
        return false;
    }
    if(year_n % 4 == 0 && year_n % 100 != 0) || (year_n % 400 == 0)
    {
        if 0 < date_n 
        {
            if m==2&&date_n<=29
            {
                return true;

            }
            else if date_n <= days_vec[m - 1]
            {
                return true;
            }
            else 
            {
                return false;
            }
            
            //break;
        }
        else
        {
            return false;
        }
    }
    else {
        if 0 < date_n &&date_n <= days_vec[m - 1]
        {
                return true;   
            //break;
        }
        else
        {
            return false;
        }
    }
 }
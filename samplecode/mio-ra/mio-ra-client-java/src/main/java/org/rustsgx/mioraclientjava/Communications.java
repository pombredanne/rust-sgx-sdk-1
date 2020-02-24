package org.rustsgx.mioraclientjava;

import com.google.gson.FieldNamingPolicy;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import org.rustsgx.mioraclientjava.bean.Datatype;
import org.rustsgx.mioraclientjava.bean.Student;
import org.rustsgx.mioraclientjava.bean.Teacher;

import java.io.BufferedReader;
import java.io.OutputStream;

public class Communications {

    public static int sendData(BufferedReader in, OutputStream out, int clientID){
        try{
            GsonBuilder gsonBuilder = new GsonBuilder();
            gsonBuilder.setFieldNamingPolicy(FieldNamingPolicy.LOWER_CASE_WITH_UNDERSCORES);
            Gson gson = gsonBuilder.create();
            for (int i=0;i<10;i++){
                Teacher teacher = new Teacher();
                Student student = new Student();
                Datatype datatype = new Datatype();
                if(i==9){
                    //send control data
                    datatype.constructDatatype("Teacher","not end",clientID);
                    getReturnData(in,i,clientID);

                    //send data body
                    teacher.constructTeacher(i,"end",clientID);
                    out.write(gson.toJson(teacher).getBytes());
                }else if (i<5){
                    //send control data
                    datatype.constructDatatype("Teacher","not end",clientID);
                    getReturnData(in,i,clientID);

                    //send data body
                    teacher.constructTeacher(i,"not end",clientID);
                    out.write(gson.toJson(teacher).getBytes());
                }else{
                    //send control data
                    datatype.constructDatatype("Student","not end",clientID);
                    getReturnData(in,i,clientID);

                    //send data body
                    student.constructStudent(i,"not end",clientID);
                    out.write(gson.toJson(student).getBytes());
                }
                //every write need wait data, if not it will make parsing error of json in sgx
                getReturnData(in,i,clientID);


            }
            return 0;
        }catch (Exception e){
            System.out.println(e.toString());
            return -1;
        }

    }

    public static void getReturnData(BufferedReader in,int i, int clientID){
        try{
            String rsp = in.readLine();
            System.out.println(rsp);
            if(rsp.equals("success")){
                System.out.printf("the %d: %d data success\n",clientID,i);
            }else{
                System.out.printf("the %d: %d data send failed\n",clientID,i);
                System.exit(0);
            }
        }catch (Exception e){
            System.out.println(e.fillInStackTrace());
            System.exit(0);
        }
    }
}

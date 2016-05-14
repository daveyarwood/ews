package ews;

import java.io.FileNotFoundException;
import java.io.FileReader;
import javax.script.ScriptEngine;
import javax.script.ScriptEngineManager;
import javax.script.ScriptException;

public class Nashorn {
  public static void main(String[] argv) {
    ScriptEngine engine = new ScriptEngineManager().getEngineByName("nashorn");

    try {
      engine.eval(new FileReader("/main.js"));
      System.exit(0);
    } catch (FileNotFoundException e) {
      System.out.println("file not found");
      System.exit(1);
    } catch (ScriptException e) {
      System.out.println("bad script");
      System.exit(1);
    }

  }
}

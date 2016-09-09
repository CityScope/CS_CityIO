import java.util.Iterator;
import java.util.Map;

public class ComparableJSONObject extends JSONObject {
  private HashMap<String, Object> values;
  
  public ComparableJSONObject() {
    super();
    values = new HashMap<String, Object>();
  }
  
  public ComparableJSONObject setObject(String name, Object value) {
    Class c = value.getClass();
    if(c.equals(Integer.class))
      setInt(name, (Integer)value);
    else if(c.equals(Long.class))
      setLong(name, (Long)value);
    else if(c.equals(Boolean.class))
      setBoolean(name, (Boolean)value);
    else if(c.equals(String.class))
      setString(name, (String)value);
    else if(c.equals(Float.class))
      setFloat(name, (Float)value);
    else if(c.equals(Double.class))
      setDouble(name, (Double)value);
    else if(c.equals(JSONArray.class))
      setJSONArray(name, (JSONArray)value);
    else
      println("ERROR: Class " + c.getName() + " is not supported.");
      //throw new Exception;
    return this;
  }
  
  @Override 
  public ComparableJSONObject setInt(String name, int value) {
    super.setInt(name, value);
    values.put(name, value);
    return this;  
  }
  
  @Override 
  public ComparableJSONObject setLong(String name, long value) {
    super.setLong(name, value);
    values.put(name, value);
    return this;  
  }
  
  @Override 
  public ComparableJSONObject setBoolean(String name, boolean value) {
    super.setBoolean(name, value);
    values.put(name, value);
    return this;  
  }
  
  @Override 
  public ComparableJSONObject setString(String name, String value) {
    super.setString(name, value);
    values.put(name, value);
    return this;  
  }
  
  @Override 
  public ComparableJSONObject setDouble(String name, double value) {
    super.setDouble(name, value);
    values.put(name, value);
    return this;  
  }
  
  @Override 
  public ComparableJSONObject setFloat(String name, float value) {
    super.setFloat(name, value);
    values.put(name, value);
    return this;  
  }
  
  @Override
  public ComparableJSONObject setJSONArray(String name, JSONArray array) {
    super.setJSONArray(name, array);
    values.put(name, array);
    return this;
  }
  
  public Object getObject(String key) {
    return values.get(key);
  }
  
  public ComparableJSONObject getDifferences(ComparableJSONObject oldState) {
    ComparableJSONObject result = new ComparableJSONObject();
    
    Iterator it = values.entrySet().iterator();
    while (it.hasNext()) {
      Map.Entry entry = (Map.Entry)it.next();
      String key = (String)entry.getKey();
      Object value = entry.getValue();
      if(!oldState.hasKey(key)) {
        result.setObject(key,value);
        continue;
      }
      Object oldValue = oldState.getObject(key);
      
      if(value.getClass().equals(JSONArray.class)) {
        
        if(!value.toString().equals(oldValue.toString())) {
          result.setObject(key,value);
        }
      }
      else if(!value.equals(oldValue)) {
        result.setObject(key, value);
      }
  }
    return result;
  }
}

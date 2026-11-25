defmodule Exceptions do
  def nested do
    try do
      try do
        try(do: work(), rescue: (_ -> recover()))
      catch
        _, _ -> recover()
      end
    rescue
      _ -> try([do: work(), catch: (_, _ -> recover())])
    catch
      _, _ -> recover()
    else
      value -> try(([{:do, value}, {:rescue, (_ -> recover())}]))
    after
      try(do: work(), rescue: (_ -> recover()))
    end
  end

  def flat do
    try do
      work()
    rescue
      _ -> recover()
    catch
      _, _ -> recover()
    end
    try do
      try(do: work(), rescue: (_ -> recover()))
    after
      work()
    end
  end

  def independent do
    try do
      inner = fn -> try(do: work(), rescue: (_ -> recover())) end
      inner.()
    rescue
      _ -> recover()
    end
  end

  def implicit do
    try(do: work(), rescue: (_ -> recover()))
  rescue
    _ -> recover()
  end
end
